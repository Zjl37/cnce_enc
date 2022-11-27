use std::error::Error;
use std::fs;
use std::collections::HashMap;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("参数不足！");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = encode(&contents);

    println!("{}", result);

    Ok(())
}

pub fn encode(text: &str) -> String {
    let cnce: HashMap<&str, &str> = HashMap::from([
        ("h", "巠"),
        ("he", "亥"),
        ("li", "里"),
        ("be", "皮"),
        ("b", "朋"),
        ("c", "炭"),
        ("n", "炎"),
        ("o", "羊"),
        ("f", "弗"),
        ("ne", "乃"),
        ("na", "内"),
        ("mg", "美"),
        ("al", "吕"),
        ("si", "圭"),
        ("p", "粦"),
        ("s", "㐬"),
        ("cl", "录"),
        ("ar", "亚"),
        ("k", "甲"),
        ("ca", "丐"),
        ("sc", "亢"),
        ("ti", "太"),
        ("v", "凡"),
        ("cr", "各"),
        ("mn", "孟"),
        ("fe", "失"),
        ("co", "古"),
        ("ni", "臬"),
        ("cu", "同"),
        ("zn", "辛"),
        ("ga", "家"),
        ("ge", "者"),
        ("as", "申"),
        ("se", "西"),
        ("br", "臭"),
        ("kr", "克"),
        ("rb", "如"),
        ("sr", "思"),
        ("y", "乙"),
        ("zr", "吿"),
        ("nb", "尼"),
        ("mo", "目"),
        ("tc", "㝵"),
        ("ru", "了"),
        ("rh", "老"),
        ("pd", "巴"),
        ("ag", "艮"),
        ("cd", "鬲"),
        ("in", "因"),
        ("sn", "易"),
        ("sb", "弟"),
        ("te", "帝"),
        ("i", "典"),
        ("xe", "山"),
        ("cs", "色"),
        ("ba", "贝"),
        ("la", "阑"),
        ("ce", "市"),
        ("pr", "普"),
        ("nd", "女"),
        ("pm", "叵"),
        ("sm", "彡"),
        ("eu", "有"),
        ("gd", "乚"),
        ("tb", "忒"),
        ("dy", "啇"),
        ("ho", "火"),
        ("er", "耳"),
        ("tm", "丢"),
        ("yb", "意"),
        ("lu", "鲁"),
        ("hf", "合"),
        ("ta", "旦"),
        ("w", "乌"),
        ("re", "来"),
        ("os", "我"),
        ("ir", "衣"),
        ("pt", "白"),
        ("au", "金"),
        ("hg", "水"),
        ("tl", "它"),
        ("pb", "铅"),
        ("bi", "必"),
        ("po", "卜"),
        ("at", "艾"),
        ("rn", "冬"),
        ("fr", "方"),
        ("ra", "雷"),
        ("ac", "阿"),
        ("th", "土"),
        ("pa", "菐"),
        ("u", "由"),
        ("np", "拿"),
        ("pu", "不"),
        ("am", "眉"),
        ("cm", "局"),
        ("bk", "咅"),
        ("cf", "開"),
        ("es", "哀"),
        ("fm", "费"),
        ("md", "门"),
        ("no", "若"),
        ("lr", "劳"),
        ("rf", "卢"),
        ("db", "杜"),
        ("sg", "喜"),
        ("bh", "波"),
        ("hs", "黑"),
        ("mt", "麦"),
        ("ds", "达"),
        ("rg", "仑"),
        ("cn", "哥"),
        ("nh", "尔"),
        ("fl", "夫"),
        ("mc", "莫"),
        ("lv", "立"),
        ("ts", "田"),
        ("og", "奥"),
    ]);

    let mut result = String::new();

    let mut cur = String::new();
    for ch in text.chars() {
        let mut next = cur.clone();
        next.push(ch);

        if next.len() >= 2 {
            if let Some(s) = cnce.get(&next.to_lowercase()[..]) {
                result += s;
                cur.clear();
                continue;
            }
        }
        if cur.len() >= 1 {
            if let Some(s) = cnce.get(&cur.to_lowercase()[..]) {
                result += s;
                cur = ch.to_string();
                continue;
            }
        }
        if let Some(ch) = next.pop() {
            result += &next;
            cur = ch.to_string();
        }
    }
    if let Some(s) = cnce.get(&cur[..]) {
        result += s;
    } else {
        for ch in cur.chars() {
            if let Some(s) = cnce.get(&ch.to_lowercase().to_string()[..]) {
                result += s;
            } else {
                result.push(ch);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let text = "Helicopter";

        assert_eq!("亥里古白耳", encode(text));
    }
}
