extern crate regex;

mod katana {
    use regex::{Regex, Captures};

    pub fn cut(origin_text: String) -> Vec<String> {
        // Remove composite abbreviations.
        let composite = Regex::new(r"(?P<comp>[a-zA-Z]{2}\s[a-zA-Z]{2})(?:\.)").unwrap();
        let mut text = composite.replace_all(origin_text.as_str(), "$comp&;&");

        // // Remove suspension points.
        let suspension = Regex::new(r"\.{3}").unwrap();
        text = suspension.replace_all(text.as_str(), "&&&.");

        // // Remove floating point numbers.
        let float_point_reg = Regex::new(r"(?P<number>[0-9]+)\.(?P<decimal>[0-9]+)").unwrap();
        text  = float_point_reg.replace_all(text.as_str(), "$number&@&$decimal");

        // Handle floats without leading zero.
        let floats_without_zeros = Regex::new(r"\s\.(?P<nums>[0-9]+)").unwrap();
        text = floats_without_zeros.replace_all(text.as_str(), " &#&$nums");

        // Remove abbreviations.
        let abbrev = Regex::new(r"(?:[A-Za-z].){2,}").unwrap();
        text = abbrev.replace_all(text.as_str(), |caps: &Captures| {
            caps.iter().map(|c| c.unwrap().replace(".", "&-&")).collect()
        });

        // Remove initials.
        let initials = Regex::new(r"(?P<init>[A-Z])(?P<point>\.)").unwrap();
        text = initials.replace_all(text.as_str(), "$init&_&");

        // Remove titles.
        let titles = Regex::new(r"(?P<title>[A-Z][a-z]{1,3})(\.)").unwrap();
        text = titles.replace_all(text.as_str(), "$title&*&");

        // Unstick sentences from each other.
        let unstick = Regex::new(r##"(?P<left>[^.?!]\.|!|\?)(?P<right>[^\s"'])"##).unwrap();
        text = unstick.replace_all(text.as_str(), "$left $right");

        // Remove sentence enders before parens
        let before_parens = Regex::new(r##"(?P<bef>[.?!])\s?\)"##).unwrap();
        text = before_parens.replace_all(text.as_str(), "&==&$bef");

        // Remove sentence enders next to quotes.
        let quote_one = Regex::new(r##"'(?P<quote>[.?!])\s?""##).unwrap();
        text = quote_one.replace_all(text.as_str(), "&^&$quote");

        let quote_two = Regex::new(r##"'(?P<quote>[.?!])\s?”"##).unwrap();
        text = quote_two.replace_all(text.as_str(), "&**&$quote");

        let quote_three = Regex::new(r##"(?P<quote>[.?!])\s?”"##).unwrap();
        text = quote_three.replace_all(text.as_str(), "&=&$quote");

        let quote_four = Regex::new(r##"(?P<quote>[.?!])\s?'""##).unwrap();
        text = quote_four.replace_all(text.as_str(), "&,&$quote");

        let quote_five = Regex::new(r##"(?P<quote>[.?!])\s?'"##).unwrap();
        text = quote_five.replace_all(text.as_str(), "&##&$quote");

        let quote_six = Regex::new(r##"(?P<quote>[.?!])\s?""##).unwrap();
        text = quote_six.replace_all(text.as_str(), "&$&$quote");

        // Split on any sentence ender.
        let s: Vec<&str> = text.split("!").collect();
        let s_last = s.len()-1;
        let mut s_one: Vec<String> = s[0..s_last].iter().map(|s| String::from(*s) + "!").collect();
        s_one.push(String::from(s[s_last])); 

        let mut s_two: Vec<String> = Vec::new();
        for sen in s_one.iter() {
            let ss: Vec<&str> =  sen.split("?").collect();
            let mut tmp_vec: Vec<String> = ss[0..ss.len()-1].iter().map(|s| String::from(*s) + "?").collect();
            s_two.append(&mut tmp_vec);
            s_two.push(String::from(ss[ss.len()-1]));
        }

        let mut final_vec: Vec<String> = Vec::new();
        for sen in s_two.iter() {
            let ss: Vec<&str> = sen.split(".").collect();
            let mut tmp_vec: Vec<String> = ss[0..ss.len()-1].iter().map(|s| String::from(*s) + ".").collect();
            final_vec.append(&mut tmp_vec);
            final_vec.push(String::from(ss[ss.len()-1]));
        }

        // Repair the damage we've done.

        // Prepare the Regexes for quote repair
        let paren_repair = Regex::new(r"&==&(?P<p>[.!?])").unwrap();

        let quote_one_repair = Regex::new(r"&\^&(?P<p>[.!?])").unwrap();
        let quote_two_repair = Regex::new(r"&\*\*&(?P<p>[.!?])").unwrap();
        let quote_three_repair = Regex::new(r"&=&(?P<p>[.!?])").unwrap();
        let quote_four_repair = Regex::new(r#"&,&(?P<p>[.!?])"#).unwrap();
        let quote_five_repair = Regex::new(r"&##&(?P<p>[.!?])").unwrap();
        let quote_six_repair = Regex::new(r"&\$&(?P<p>[.!?])").unwrap();

        let mut results: Vec<String> = final_vec.iter()
            .map(|s| {
                // Skip whitespace zones.
                s.trim()
                 // Repair composite abbreviations.
                 .replace("&;&", ".")
                 // Repair suspension points.
                 .replace("&&&", "..")
                 // Repair Floats.
                 .replace("&@&", ".")
                 // Repair floats without leading zeros
                 .replace("&#&", ".")
                 // Repair abbreviations.
                 .replace("&-&", ".")
                 // Repair intials.
                 .replace("&_&", ".")
                 // Repair titles.
                 .replace("&*&", ".")
            })
            .map(|s| {
                paren_repair.replace_all(s.as_str(), r"$1)")
            })
            // Repair quotes with sentence enders.
            .map(|s| {
                quote_one_repair.replace_all(s.as_str(), r#"'$p""#)
            })
            .map(|s| {
                quote_two_repair.replace_all(s.as_str(), r#"'$p”"#)
            })
            .map(|s| {
                 quote_three_repair.replace_all(s.as_str(), r#"$p”"#)
            })
            .map(|s| {
                quote_four_repair.replace_all(s.as_str(), r#"'""#)
            })
            .map(|s| {
                quote_five_repair.replace_all(s.as_str(), r#"$p'"#)
            })
            .map(|s| {
                quote_six_repair.replace_all(s.as_str(), r#"$p""#)
            })
            .filter(|s| s.len() > 1)
            .collect();

        results
    }
}

#[cfg(test)]
mod test {
    use super::katana;
    use regex::{Regex, Captures};

    #[test]
    fn it_works() {
        let text = String::from("For years, people in the U.A.E.R. have accepted murky air, tainted waters and scarred landscapes as the unavoidable price of the country’s meteoric economic growth. But public dissent over environmental issues has been growing steadily in the communist nation, and now seems to be building the foundations of a fledgling green movement! In July alone, two separate demonstrations made international news when they turned violent after about 1.5 minutes... These recent successes come after a slew of ever-larger and more violent green protests over the past few years, as the environmentalist Dr. C. Jeung of China’s growth becomes harder to ignore.Some ask: “Are demonstrations are evidence of the public anger and frustration at opaque environmental management and decision-making?” Others yet say: \"Should we be scared about these 'protests'?\" The man made a quick calculation and found the result to be .625. (This is another sentence in parens.) This is the last sentence.");

        let result = vec![
            "For years, people in the U.A.E.R. have accepted murky air, tainted waters and scarred landscapes as the unavoidable price of the country’s meteoric economic growth.",
            "But public dissent over environmental issues has been growing steadily in the communist nation, and now seems to be building the foundations of a fledgling green movement!",
            "In July alone, two separate demonstrations made international news when they turned violent after about 1.5 minutes...",
            "These recent successes come after a slew of ever-larger and more violent green protests over the past few years, as the environmentalist Dr. C. Jeung of China’s growth becomes harder to ignore.",
            "Some ask: “Are demonstrations are evidence of the public anger and frustration at opaque environmental management and decision-making?”",
            "Others yet say: \"Should we be scared about these 'protests'?\"",
            "The man made a quick calculation and found the result to be .625.",
            "(This is another sentence in parens.)",
            "This is the last sentence.",
        ];

        assert_eq!(result, katana::cut(text));
    }
    
    #[test]
    fn test_regex() {
        let text = "i.e. Marca et al. good";
        let abbrev = Regex::new(r"(?P<comp>[a-zA-Z]{2}\s[a-zA-Z]{2})(?:\.)").unwrap();
        let result = abbrev.replace_all(text, "$comp&;&");

        assert_eq!("i.e. Marca et al&;& good", result);
    }
}
