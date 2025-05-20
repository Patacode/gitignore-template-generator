# Binary Crate Documentation

Available options:

```text
Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

Generate templates for .gitignore files

Arguments:
  [TEMPLATE_NAMES]...  A non-empty list of gitignore template names

Options:
  -c, --check                          Enable robust template names check
  -g, --generator-uri <GENERATOR_URI>  The template generator uri [default: /developers/gitignore/api]
  -l, --list                           List available templates
  -i, --lister-uri <LISTER_URI>        The template lister uri [default: /developers/gitignore/api/list]
  -s, --server-url <SERVER_URL>        The template manager url [default: https://www.toptal.com]
  -t, --timeout <TIMEOUT>              The template generation and listing service calls timeout [default: 5s/5000ms]
  -u, --timeout-unit <TIMEOUT_UNIT>    The timeout unit [default: second] [possible values: millisecond, second]
  -h, --help                           Print help
  -V, --version                        Print version
  -a, --author                         Print author

Version: 0.10.0
Author: Patacode <pata.codegineer@gmail.com>
```

The CLI tool is a simple API binder to toptal gitignore template generation
service. It takes gitignore template names as positional arguments and
generates a gitignore template for all the provided arguments you can reuse
in your project:

```text
$ gitignore-template-generator rust python java
# ...
# some template for a project in rust, python and java
# ...
```

Behind the scene, it calls the template generator service as pointed to by
the [-g --generator-uri](#-g-generator-uri) option.

Positional arguments cannot contains commas (`,`) nor white spaces (as defined
in the [Unicode Character Database](https://www.unicode.org/reports/tr44)
[`PropList.txt`](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)):

```text
$ gitignore-template-generator "rust," "python"
error: invalid value 'rust,' for '[TEMPLATE_NAMES]...': Commas are not allowed in template names

For more information, try '--help'.
$ gitignore-template-generator "rus t" "python"
error: invalid value 'rus t' for '[TEMPLATE_NAMES]...': Whitespace characters are not allowed in template names

For more information, try '--help'.
```

All the other CLI options are optionals. See below sections for detailed
explanations of each:

- [-c --check](#-c-check)
- [-g --generator-uri](#-g-generator-uri)
- [-l --list](#-l-list)
- [-i --lister-uri](#-i-lister-uri)
- [-s --server-url](#-s-server-url)
- [-t --timeout](#-t-timeout)
- [-u --timeout-unit](#-u-timeout-unit)
- [-h --help](#-h-help)
- [-V --version](#-V-version)
- [-a --author](#-a-author)

## -c --check

This option is a **boolean** option that, when set, enables robust template
names check, meaning it will ensure that all provided template names are
valid (i.e. supported by the template manager service):

```text
& gitignore-template-generator rust pyth javaa --check
Following template names are not supported: pyth, javaa. For the list of available template names, try '--list'.
```

Behind the scene, the template lister service will be called to check for any
unsupported template names, thus, a slight overhead might be expected.

This option comes in handy whenever the underlying template manager service does
not provide meaningful error message for invalid template names, and you want
to ensure all provided values are valid.

With the default template manager service (i.e. toptal), a 404
is returned for invalid template names, but no meaningful error message. So,
without the `-c/--check` option, the resulting output would be:

```text
& gitignore-template-generator rust pyth javaa
An error occurred during the API call: http status: 404
```

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --check
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --check <TEMPLATE_NAMES>...

For more information, try '--help'.
```

As well, since it's a **boolean** option, it does not take any value:

```text
$ gitignore-template-generator "rust" "pytho" --check="true"
error: unexpected value 'true' for '--check' found; no more were expected

Usage: gitignore-template-generator --check [TEMPLATE_NAMES]...

For more information, try '--help'.
```

Be noted that the following is valid:

```text
& gitignore-template-generator --check rust pyth javaa
```

As its a **boolean** option, it does not take into account any following
positional arguments, but act more like a flag.

Only with the `=` syntax one can try to provide a value to it, but will result
in an error.

And cannot be specified multiple times:

```text
$ gitignore-template-generator rust pyth javaa --check --check
error: the argument '--check' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -g --generator-uri

This option allows you to set a custom template generator uri. It takes a string
value and defaults to `/developers/gitignore/api` if not provided, which
correspond to the gitignore template generator uri of portal service:

```text
$ gitignore-template-generator rust python --generator-uri /developers/gitignore/api
# ...
# some rust python template
# ...
```

This endpoint takes one path parameter as defined in
`https://www.toptal.com/developers/gitignore/api/{templateNames}`, with
`{templateNames}` being a comma-separated list of template names.

It isn't really useful when used alone, as default template manager service is
toptal and it only supports `/developers/gitignore/api` uri to generate
gitignore templates. But it comes in handy in combination with
the [-s --server-url](#-s-server-url) option, as it allows you to make it
point to some custom API:

```text
$ gitignore-template-generator rust python \
    --generator-uri /gitignore/generate \
    --server-url https://myapis.foobar.com
```

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --generator-uri /developers/gitignore/api
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --generator-uri <GENERATOR_URI> <TEMPLATE_NAMES>...

For more information, try '--help'.
```

It must also start with a `/`:

```text
$ gitignore-template-generator --generator-uri developers/gitignore/api
error: invalid value 'developers/gitignore/api' for '[GENERATOR_URI]...': URIs must start a slash (/)

For more information, try '--help'.
```

Depending on how the underlying template manager service handles undefined
endpoints, if an undefined uri is provided, the resulting output would be:

```text
$ gitignore-template-generator rust -generator-uri /foo
An error occurred during the API call: http status: 404
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator rust python --generator-uri /developers/gitignore/api --generator-uri /developers/gitignore/api
error: the argument '--generator-uri <GENERATOR_URI>' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -l --list

This option is a **boolean** option that, when set, will list all the available
template names:

```text
$ gitignore-template-generator --list
template1
template2
template3
template4
...
```

It will be listed one per line, which makes it really useful in combination with
less or grep:

```text
$ gitignore-template-generator --list | grep ja
django
jabref
janet
java
ninja
$ gitignore-template-generator --list | less
# in some interactive prompt
...
template1
template2
template3
template4
...
:
```

Behind the scene, it calls the template lister service as pointed to by the
[-i --lister-uri](#-i-lister-uri) option.

If the following options are provided and this option is set, they will be
skipped:

- [-c --check](#-c-check)
- [-g --generator-uri](#-g-generator-uri)

Same applies for positional arguments, if some are given, and this option is
set, they will be skipped.

And cannot be specified multiple times:

```text
$ gitignore-template-generator --list --list
error: the argument '--list' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -i --lister-uri

This option allows you to set a custom template lister uri. It takes a string
value and defaults to `/developers/gitignore/api/list` if not provided, which
correspond to the gitignore template lister uri of portal service:

```text
$ gitignore-template-generator --list --lister-uri /developers/gitignore/api/list
template1
template2
template3
template4
...
```

Of course, this option only makes sense in the context of
[-l --list](#-l-list) or [-c --check](#-c-check) options. With the later
coming from the fact that robust template names check relies on the lister
service.

Moreover, it isn't really useful when used alone, as default template manager
service is toptal and it only supports `/developers/gitignore/api/list` uri to
list available gitignore templates. But it comes in handy in combination with
the [-s --server-url](#-s-server-url) option, as it allows you to make it
point to some custom API:

```text
$ gitignore-template-generator --list \
    --lister-uri /gitignore/list \
    --server-url https://myapis.foobar.com
```

If we take a more complex example, one could combine this option with template
generation-related options to get a gitignore template with robust template
names check using a custom API:

```text
$ gitignore-template-generator rust pyth java --check \
    --lister-uri /gitignore/list \
    --generator-uri /gitignore/generate \
    --server-url https://myapis.foobar.com
```

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --lister-uri /developers/gitignore/api/list
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --lister-uri <LISTER_URI> <TEMPLATE_NAMES>...

For more information, try '--help'.
```

It must also start with a `/`:

```text
$ gitignore-template-generator --lister-uri developers/gitignore/api/list
error: invalid value 'developers/gitignore/api/list' for '[GENERATOR_URI]...': URIs must start a slash (/)

For more information, try '--help'.
```

Depending on how the underlying template manager service handles undefined
endpoints, if an undefined uri is provided, the resulting output would be:

```text
$ gitignore-template-generator --list --lister-uri /foo
An error occurred during the API call: http status: 404
```

Same result would happen if template generation with robust template names check
enabled:

```text
$ gitignore-template-generator rust pyth --check --lister-uri /foo
An error occurred during the API call: http status: 404
```

And cannot be specified multiple times:

```text
--list --lister-uri /developers/gitignore/api/list
$ gitignore-template-generator --list --lister-uri /developers/gitignore/api/list --lister-uri /developers/gitignore/api/list
error: the argument '--lister-uri <LISTER_URI>' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -s --server-url

This option allows you to set a custom template manager base url. It takes a string
value and defaults to `https://www.toptal.com` if not provided, which
correspond to the toptal base url, where their gitignore template generation
service is hosted:

```text
$ gitignore-template-generator rust python --server-url https://myapis.foobar.com
# ...
# some rust python template
# ...
```

It comes in handy if you want the tool to use your own custom gitignore
template generation service. And is pretty useful when combined with
[-g --generator-uri](#-g-generator-uri) and/or [-i --lister-uri](#-i-lister-uri)
options, as it allows to customize the hit endpoint uris, unless you rely on
the defaults.

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --server-url https://myapis.foobar.com
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --server-url <SERVER_URL> <TEMPLATE_NAMES>...

For more information, try '--help'.
```

It must also be a valid URL:

```text
$ gitignore-template-generator --server-url htps:/myapis.foobar.com
error: invalid value 'htps:/myapis.foobar.com' for '[SERVER_URL]...': Value must be a valid URL

For more information, try '--help'.
```

If an inexistent server is provided, the result would be:

```text
$ gitignore-template-generator rust python --server-url https://myapis.foobar.com
An error occurred during the API call: io: failed to lookup address information: Name or service not known
```

And cannot be specified multiple times:

```text
--list --lister-uri /developers/gitignore/api/list
$ gitignore-template-generator rust python --server-url https://myapis.foobar.com --server-url https://myapis.foobar.com
error: the argument '--server-url <SERVER_URL>' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -t --timeout

This option allows you to change the service calls timeout. It takes an unsigned
integer value and, if not provided, conditionally defaults to `5` if
[-u --timeout-unit](#-u-timeout-unit) is set to `second` (i.e. 5 seconds),
else to `5000` (i.e. 5000 milliseconds):

```text
$ gitignore-template-generator rust python --timeout 4
# ...
# some rust python template
# ...
```

The conditional default value allows to prevent timeout issues if
[-u --timeout-unit](#-u-timeout-unit) is set to `millisecond` alone:

```text
$ gitignore-template-generator rust python --timeout-unit millisecond
# ...
# some rust python template
# ...
```

This option comes in handy to limit polling time if the generator is really
way too slow.

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --timeout 4
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --timeout <TIMEOUT> <TEMPLATE_NAMES>...

For more information, try '--help'.
```

It must also be a positive integer:

```text
$ gitignore-template-generator --timeout dd
error: invalid value 'dd' for '--timeout <TIMEOUT>': invalid digit found in string

For more information, try '--help'.
$ gitignore-template-generator --timeout="-1"
error: invalid value '-1' for '--timeout <TIMEOUT>': invalid digit found in string

For more information, try '--help'.
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator rust python --timeout 4 --timeout 6
error: the argument '--timeout <TIMEOUT>' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -u --timeout-unit

This option allows you to change the timeout unit. It takes a string value
defined in [second, millisecond] and defaults to `second` if not provided:

```text
$ gitignore-template-generator rust python --timeout-unit millisecond
# ...
# some rust python template
# ...
```

It isn't really useful when used alone, but comes pretty useful in combination
with [-t --timeout](#-t-timeout) option if you prefer to work with millisecond
units instead of seconds.

Naturally, this option cannot be provided without positional arguments:

```text
$ gitignore-template-generator --timeout-unit millisecond
error: the following required arguments were not provided:
  <TEMPLATE_NAMES>...

Usage: gitignore-template-generator --timeout-unit <TIMEOUT_UNIT> <TEMPLATE_NAMES>...

For more information, try '--help'.
```

It must be set to a valid value from supported set:

```text
$ gitignore-template-generator rust python --timeout-unit millisecondd
error: invalid value 'millisecondd' for '--timeout-unit <TIMEOUT_UNIT>'
  [possible values: millisecond, second]

For more information, try '--help'.
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator rust python --timeout-unit millisecond --timeout-unit second
error: the argument '--timeout-unit <TIMEOUT_UNIT>' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -h --help

This option is a preemptive **boolean** option that, when set, will display
a help message, listing you available options with short description for
each:

```text
$ gitignore-template-generator --help
# ...
# help output
# ...
```

By being preemptive, it means any other arguments, either named or positional,
will be skipped:

```text
$ gitignore-template-generator rust python \
    --check
    --lister-uri /foo
    --generator-uri /bar
    --server-url https://myapis.foobar.com
    --help
# ...
# help output
# ...
```

It's a special kinda 'default' option coming from virtually all cli tools,
along with [-V --version](#-V-version) and [-a --author](#-a-author) options.

It has precedence over [-V --version](#-V-version) and [-a --author](#-a-author)
options if specified along:

```text
$ gitignore-template-generator -hVa
# ...
# help output
# ...
```

It does not take any value:

```text
$ gitignore-template-generator --help="true"
error: unexpected value 'true' for '--help' found; no more were expected

Usage: gitignore-template-generator --help [TEMPLATE_NAMES]...

For more information, try '--help'.
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator rust python -hh
error: the argument '--help' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -V --version

This option is a preemptive **boolean** option that, when set, will display
version information:

```text
$ gitignore-template-generator --version
gitignore-template-generator 0.10.0
```

By being preemptive, it means any other arguments, either named or positional,
will be skipped:

```text
$ gitignore-template-generator rust python \
    --version
    --lister-uri /foo
    --generator-uri /bar
    --server-url https://myapis.foobar.com
    --help
gitignore-template-generator 0.10.0
```

It's a special kinda 'default' option coming from virtually all cli tools,
along with [-h --help](#-h-help) and [-a --author](#-a-author) options.

It has precedence over [-a --author](#-a-author) options if specified along:

```text
$ gitignore-template-generator -Va
gitignore-template-generator 0.10.0
```

It does not take any value:

```text
$ gitignore-template-generator --version="true"
error: unexpected value 'true' for '--version' found; no more were expected

Usage: gitignore-template-generator --version [TEMPLATE_NAMES]...

For more information, try '--help'.
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator -VV
error: the argument '--version' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```

## -a --author

This option is a preemptive **boolean** option that, when set, will display
author information:

```text
$ gitignore-template-generator --version
Patacode <pata.codegineer@gmail.com>
```

By being preemptive, it means any other arguments, either named or positional,
will be skipped:

```text
$ gitignore-template-generator rust python \
    --author
    --lister-uri /foo
    --generator-uri /bar
    --server-url https://myapis.foobar.com
    --help
Patacode <pata.codegineer@gmail.com>
```

It's a special kinda 'default' option coming from virtually all cli tools,
along with [-h --help](#-h-help) and [-V --version](#-V-version) options.

It does not take any value:

```text
$ gitignore-template-generator --author="true"
error: unexpected value 'true' for '--author' found; no more were expected

Usage: gitignore-template-generator --author [TEMPLATE_NAMES]...

For more information, try '--help'.
```

And cannot be specified multiple times:

```text
$ gitignore-template-generator -aa
error: the argument '--author' cannot be used multiple times

Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

For more information, try '--help'.
```
