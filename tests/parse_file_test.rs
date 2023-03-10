use regex::Regex;
use hparse::{HParse, ParseFile};

mod common;

#[test]
fn parse_parse_file_test() {
    let yaml = common::example("gallery_parse_file.yaml");
    let res = serde_yaml::from_str::<ParseFile>(&yaml);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn parse_use_callback() {
    let gallery = r#"
        <tr data-new="1"><td class="gl1e" style="width:250px"><div style="height:327px;width:250px"><a href="https://e-hentai.org/g/2488708/ff09229178/"><img style="height:327px;width:250px" alt="Lapis Strip Game [Eromela Art]" title="Lapis Strip Game [Eromela Art]" src="https://ehgt.org/c4/0c/c40c25c1a408159b80aee533468aabe27f36b896-259620-1680-2195-jpg_250.jpg"></a></div></td><td class="gl2e"><div><div class="gl3e"><div class="cn cta" onclick="document.location='https://e-hentai.org/western'">Western</div><div class="glnew" onclick="popUp('https://e-hentai.org/gallerypopups.php?gid=2488708&amp;t=ff09229178&amp;act=addfav',675,415)" id="posted_2488708">2023-03-09 12:27</div><div class="ir" style="background-position:-80px -1px;opacity:0.33333333333333"></div><div><a href="https://e-hentai.org/uploader/TallHatMan">TallHatMan</a></div><div>5 pages</div><div class="gldown"><img src="https://ehgt.org/g/td.png" alt="T" title="No torrents available"></div></div><a href="https://e-hentai.org/g/2488708/ff09229178/"><div class="gl4e glname" style="min-height:335px"><div class="glink">Lapis Strip Game [Eromela Art]</div><div><table><tbody><tr><td class="tc">language:</td><td><div class="gt" title="language:english">english</div></td></tr><tr><td class="tc">parody:</td><td><div class="gtl" title="parody:steven universe">steven universe</div></td></tr><tr><td class="tc">character:</td><td><div class="gtl" title="character:lapis lazuli">lapis lazuli</div></td></tr></tbody></table></div></div></a></div></td></tr>
    "#;
    let yaml = common::example("gallery_parse_file.yaml");
    let parse = HParse::new(&yaml).unwrap();

    let json = parse.json(gallery, Some(Box::new(
        |tag, s, file| {
            const PATTERN_RATING: &str = r#"(\d+)px"#;
            let reg = Regex::new(PATTERN_RATING).unwrap();
            let mut value = 5 as f32;

            let captures = reg.captures(s).ok_or(String::from(""))?;
            let n1 = captures[1].parse::<i32>()?;
            let n2 = captures[2].parse::<i32>()?;

            value -= (n1 / 16) as f32;
            if n2 == 21 {
                value -= 0.5 as f32;
            }

            Ok(value.to_string())
        }
    )));

    assert_eq!(json.is_ok(), true);
}
