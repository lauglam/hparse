use hparse::actions::{Action, AnyOfAction, AttributeAction, ChooseAction, RegexAction, SelectAction, StrAction};

#[test]
fn parse_actions_test() {
    let yaml = r#"
        - !Select
          selector: !AnyOf
            - ".glname a"
            - ".gl2e a"
          description: |
            1. Minimal|MinimalPlus|Compact|Thumbnail: `.glname a`
            2. Extended: `.gl2e a`
          exception: '元素: `.glname a`或`.gl2e a`未找到'
        - !Attribute
          attr: !Single href
          exception: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<Vec<Action>>(yaml);
    assert_eq!(res.is_ok(), true);
}

// AnyOfAction test

#[test]
fn parse_any_of_action_test() {
    let yaml = r#"
        actions:
          - !Regex
            regex: !Single <img[^>]*style="height:(\d+)px;width:(\d+)px[^"]*"[^>]*src="([^"]+)"
            group: 2

          - !Regex
            regex: !Single width:(\d+)px; height:(\d+)px.+?url\((.+?)\)
            group: 1
        description: |
          Attention:
          This is the `actions` field in the `AayOfAction`, not the `actions` field in the `ParseFile`
        exception: All Actions have no return value
    "#;

    let res = serde_yaml::from_str::<AnyOfAction>(yaml);
    assert_eq!(res.is_ok(), true);
}

// AttributeAction test

#[test]
fn parse_attribute_action_test() {
    let yaml = r#"
        attr: !Single href
        description: href attribute action
        exception: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<AttributeAction>(yaml);
    assert_eq!(res.is_ok(), true);

    let yaml = r#"
        attr: !AnyOf
          - href
          - title
        description: href attribute action
        exception: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<AttributeAction>(yaml);
    assert_eq!(res.is_ok(), true);
}

// ChooseAction test

#[test]
fn parse_choose_action_test() {
    let yaml = r##"
        keys:
          - Doujinshi
          - Manga
          - Artist CG
          - Game CG
          - Western
          - Non-H
          - Image Set
          - Cosplay
          - Asian Porn
          - Misc
        values:
          - "#fc4e4e"
          - "#e78c1a"
          - "#c7bf07"
          - "#1a9317"
          - "#5dc13b"
          - "#0f9ebd"
          - "#2756aa"
          - "#8800c3"
          - "#b452a5"
          - "#707070"
        description: the description of choose action
        exception: The value entered has no matching options
    "##;

    let res = serde_yaml::from_str::<ChooseAction>(yaml);
    assert_eq!(res.is_ok(), true);
}

// FuncAction test

#[test]
fn parse_func_action_test() {}

// RegexAction test

#[test]
fn parse_regex_action_test() {
    let yaml = r#"
        regex: !Single https?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
        group: 1
        description: description of regex action
        exception: '字段: `gid`在正则表达式匹配时出错'
    "#;

    let res = serde_yaml::from_str::<RegexAction>(yaml);
    assert_eq!(res.is_ok(), true);

    let yaml = r#"
        regex: !AnyOf
          - https?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
          - http?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
        group: 1
        description: description of regex action
        exception: '字段: `gid`在正则表达式匹配时出错'
    "#;

    let res = serde_yaml::from_str::<RegexAction>(yaml);
    assert_eq!(res.is_ok(), true);
}

// SelectAction test

#[test]
fn parse_select_action_test() {
    let yaml = r#"
        selector: !Single img
        description: description of select action
        exception: 'element: `img` not found'
    "#;

    let res = serde_yaml::from_str::<SelectAction>(yaml);
    assert_eq!(res.is_ok(), true);

    let yaml = r#"
        selector: !AnyOf
          - img
          - a
        description: description of select action
        exception: 'element: `img` not found'
    "#;

    let res = serde_yaml::from_str::<SelectAction>(yaml);
    assert_eq!(res.is_ok(), true);
}

// StrAction test

#[test]
fn parse_str_action_test() {
    let yaml = r#"
        description: description of text action
        exception: string empty
    "#;

    let res = serde_yaml::from_str::<StrAction>(yaml);
    assert_eq!(res.is_ok(), true);
}
