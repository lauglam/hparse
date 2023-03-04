use hparse::ParseFile;

mod common;

#[test]
fn parse_parse_file_test() {
    let yaml = common::example("gallery_parse_file.yaml");
    let res = serde_yaml::from_str::<ParseFile>(&yaml);
    assert_eq!(res.is_ok(), true);
}
