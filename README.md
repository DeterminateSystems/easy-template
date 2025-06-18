# easy-template

An easy templating CLI that takes a [Handlebars] template file and a data file (JSON, YAML, and TOML are supported) and spits out a formatted string.

```shell
easy-template README.template.md inputs.yaml > README.md
```

Working examples in this repo:

```shell
# JSON
easy-template ./examples/README.template.md ./examples/data.json

# YAML
easy-template ./examples/README.template.md ./examples/data.yaml

# TOML
easy-template ./examples/README.template.md ./examples/data.toml
```

[handlebars]: https://handlebarsjs.com
