<h1 align="center">

<strong>HParse</strong>

</h1>

A library for parsing `HTML` by writing yaml `parse_file`.

## Usage

:construction: coming soon!

### Example: create [AttributeAction](#action--attributeaction)

Since `attr` has two [variant](#variable), you need to indicate(`!Single`or`!AnyOf`) its [variant](#variable).

```yaml
attr: !Single href
description: href attribute action
exception: 'attribute: `href`not found'
```

or

```yaml
attr: !AnyOf
  - href
  - title
description: href attribute action
exception: 'attribute: `href`not found'
```

### Example: create [ChooseAction](#action--chooseaction)

```yaml
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
exception: the value entered has no matching options
```

### Example: create [CallbackAction](#action--callbackaction)

:construction: coming soon!

### Example: create [RegexAction](#action--regexaction)

Since `regex` has two [variant](#variable), you need to indicate(`!Single`or`!AnyOf`) its [variant](#variable).

```yaml
regex: !Single https?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
group: 1
description: description of regex action
exception: 'field: 'gid' error in regular expression matching'
```

or

```yaml
regex: !AnyOf
  - https?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
  - http?://(?:exhentai.org|e-hentai.org|lofi.e-hentai.org)/(?:g|mpv)/(\d+)/([0-9a-f]{10})
group: 1
description: description of regex action
exception: 'field: 'gid' error in regular expression matching'
```

### Example: create [SelectAction](#action--selectaction)

Since `selector` has two [variant](#variable), you need to indicate(`!Single`or`!AnyOf`) its [variant](#variable).

```yaml
selector: !Single img
description: description of select action
exception: 'element: `img` not found'
```

or

```yaml
selector: !AnyOf
  - img
  - a
description: description of select action
exception: 'element: `img` not found'
```

### Example: create [StrAction](#action--straction)

```yaml
description: description of text action
exception: string empty
```

### Example: create [AnyOfAction](#action--anyofaction)

Because `actions` contain different action, you need to indicate the [variant](#action) of action.

Since `regex` also has two [variant](#variable), you need to indicate its [variant](#variable) as well.

```yaml
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
```

### Example: create ParseFile

```yaml

```

## Structures

### Action: AnyOfAction

| field       | type          | required | description                                                         |
|-------------|---------------|----------|---------------------------------------------------------------------|
| actions     | array(Action) | true     |                                                                     |
| description | string        | false    | describe this `action`                                              |
| exception   | string        | false    | this `text` will appear in the `error` message when an error occurs |

### Action: AttributeAction

| field       | type                  | required | description                                                         |
|-------------|-----------------------|----------|---------------------------------------------------------------------|
| attr        | [Variable](#variable) | true     |                                                                     |
| description | string                | false    | describe this `action`                                              |
| exception   | string                | false    | this `text` will appear in the `error` message when an error occurs |

### Action: ChooseAction

| field       | type          | required | description                                                         |
|-------------|---------------|----------|---------------------------------------------------------------------|
| keys        | array(string) | true     |                                                                     |
| values      | array(string) | true     |                                                                     |
| description | string        | false    | describe this `action`                                              |
| exception   | string        | false    | this `text` will appear in the `error` message when an error occurs |

### Action: CallbackAction

| field       | type   | required | description            |
|-------------|--------|----------|------------------------|
| tag         | string | true     |                        |
| description | string | false    | describe this `action` |

### Action: RegexAction

| field       | type                  | required | description                                                         |
|-------------|-----------------------|----------|---------------------------------------------------------------------|
| regex       | [Variable](#variable) | true     |                                                                     |
| group       | number                | true     |                                                                     |
| description | string                | false    | describe this `action`                                              |
| exception   | string                | false    | this `text` will appear in the `error` message when an error occurs |

### Action: SelectAction

| field       | type                  | required | description                                                            |
|-------------|-----------------------|----------|------------------------------------------------------------------------|
| selector    | [Variable](#variable) | true     | css selector  [more](https://developer.mozilla.org/en-US/docs/Web/CSS) |
| description | string                | false    | describe this `action`                                                 |
| exception   | string                | false    | this `text` will appear in the `error` message when an error occurs    |

### Action: StrAction

| field       | type   | required | description                                                         |
|-------------|--------|----------|---------------------------------------------------------------------|
| description | string | false    | describe this `action`                                              |
| exception   | string | false    | this `text` will appear in the `error` message when an error occurs |

### Variable

| kind   | description                  |
|--------|------------------------------|
| Single | single string value          |
| AnyOf  | any of several string values |

### Action

| kind      | action                                      | description |
|-----------|---------------------------------------------|-------------|
| AnyOf     | [AnyOfAction](#action--anyofaction)         |             |
| Attribute | [AttributeAction](#action--attributeaction) |             |
| Choose    | [ChooseAction](#action--chooseaction)       |             |
| Callback  | [CallbackAction](#action--callbackaction)   |             |
| Regex     | [RegexAction](#action--regexaction)         |             |
| Select    | [SelectAction](#action--selectaction)       |             |
| Str       | [StrAction](#action--straction)             |             |

### ActionError

| field   | type                                | required | description |
|---------|-------------------------------------|----------|-------------|
| kind    | [ActionErrorKind](#actionerrorkind) | true     |             |
| message | string                              | false    |             |

### ActionErrorKind

| kind                    | description                                                    |
|-------------------------|----------------------------------------------------------------|
| AnyActionAllActionFail  | the error kind for [AnyOfAction](#action--anyofaction)         |
| AttributeNotFound       | the error kind for [AttributeAction](#action--attributeaction) |
| MissingCallbackFunction | the error kind for [CallbackAction](#action--callbackaction)   |
| PatternNotCovered       | the error kind for [ChooseAction](#action--chooseaction)       |
| RegexNotMatch           | the error kind for [RegexAction](#action--regexaction)         |
| ElementNotFound         | the error kind for [SelectAction](#action--selectaction)       |
| StrEmpty                | the error kind for [StrAction](#action--straction)             |

## Dependencies

- visdom: [https://github.com/fefit/visdom](https://github.com/fefit/visdom)
- serde: [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde)
- serde-yaml: [https://github.com/dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml)

## License

[MIT License](./LICENSE).
