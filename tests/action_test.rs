use hparse::actions::{Action, AttrAction};

#[test]
fn parse_actions_test() {
    let actions = r#"
      - !Select
        selector: !AnyOf
          - ".glname a"
          - ".gl2e a"
        description: |
          1. Minimal|MinimalPlus|Compact|Thumbnail: `.glname a`
          2. Extended: `.gl2e a`
        error:
          kind: ElementNotFound
          date: '2023-03-02'
          message: '元素: `.glname a`或`.gl2e a`未找到'
      - !Attribute
        attr: !Single href
        error:
          kind: AttributeNotFound
          date: '2023-03-02'
          message: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<Vec<Action>>(actions);
    assert_eq!(res.is_ok(), true);
}

// AnyOfAction test

#[test]
fn parse_any_of_action_test() {
    let actions = r#"
      - !Select
        selector: !AnyOf
          - ".glname a"
          - ".gl2e a"
        description: |
          1. Minimal|MinimalPlus|Compact|Thumbnail: `.glname a`
          2. Extended: `.gl2e a`
        error:
          kind: ElementNotFound
          date: '2023-03-02'
          message: '元素: `.glname a`或`.gl2e a`未找到'

      - !Attribute
        attr: !Single href
        error:
          kind: AttributeNotFound
          date: '2023-03-02'
          message: '属性: `href`未找到'

      - !AnyOf
        actions:
          - !Regex
            regex: !Single <img[^>]*style="height:(\d+)px;width:(\d+)px[^"]*"[^>]*src="([^"]+)"
            group: 2

          - !Regex
            regex: !Single width:(\d+)px; height:(\d+)px.+?url\((.+?)\)
            group: 1
        description: |
          Attention:
          This is the `actions` field in the `AanyOfAction`, not the `actions` field in the `PaseFile`
    "#;

    let res = serde_yaml::from_str::<Vec<Action>>(actions);
    assert_eq!(res.is_ok(), true);
}

// AttrAction test

#[test]
fn parse_attr_action_test() {
    let attr_action = r#"
      attr: !Single href
      description: href attribute action
      error:
        kind: AttributeNotFound
        message: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<AttrAction>(attr_action);
    assert_eq!(res.is_ok(), true);

    let attr_action = r#"
      attr: !AnyOf
        - href
        - title
      description: href attribute action
      error:
        kind: AttributeNotFound
        message: '属性: `href`未找到'
    "#;

    let res = serde_yaml::from_str::<AttrAction>(attr_action);
    assert_eq!(res.is_ok(), true);
}

// ChooseAction test

#[test]
fn parse_choose_action_test() {}

// FuncAction test

#[test]
fn parse_func_action_test() {}

// RegAction test

#[test]
fn parse_reg_action_test() {}

// SelectAction test

#[test]
fn parse_select_action_test() {}

// StrAction test

#[test]
fn parse_str_action_test() {}
