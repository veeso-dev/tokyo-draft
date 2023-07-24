# tokyo-draft

<p align="center">~ A webservice to render HTML templates from API data ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://crates.io/crates/tokyo-draft" target="_blank">Crates.io</a>
</p>
<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.1 (24/07/2023)</p>

<p align="center">
  <a href="https://opensource.org/license/mit/"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso-dev/tokyo-draft/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso-dev/tokyo-draft.svg"
      alt="Repo stars"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso-dev/tokyo-draft/actions"
    ><img
      src="https://github.com/veeso-dev/tokyo-draft/workflows/build-test/badge.svg"
      alt="Linux CI"
  /></a>
</p>

---

- [tokyo-draft](#tokyo-draft)
  - [About tokyo-draft](#about-tokyo-draft)
  - [Get started](#get-started)
    - [Run](#run)
  - [tokyo-draft API](#tokyo-draft-api)
    - [Check](#check)
    - [Render](#render)
      - [Errors](#errors)
  - [Hook API](#hook-api)
  - [Support the developer](#support-the-developer)
  - [Contributing and issues](#contributing-and-issues)
  - [Changelog](#changelog)
  - [License](#license)

---

## About tokyo-draft

tokyo-draft is a Rust web service which renders HTML templates starting from incoming data from API. The API allows you to render any template you've pre-configured in the service configuration.
The rendered template is then sent via a POST request to a configurable hook.

---

## Get started

### Run

First configure the environment file as you wish, then source .env and run tokyo-draft with

```sh
./tokyo-draft.sh start /var/run/tokyo-draft.pid
```

At this point tokyo-draft will be served on the specified port in the `.env` (or `.env.override`) file. (Default: `3011`)

## tokyo-draft API

### Check

Check web service status:

```txt
GET /check
```

Response:

```json
{
  "status": "ok"
}
```

### Render

Render a template with provided data:

```txt
POST /render
```

payload:

```json
{
  "template": "my-template",
  "data": {
    "key1": "value",
    "key2": 123,
  },
  "hookMetadata": {
    /* custom data to be passed to hook */
  }
}
```

Response (HTML):

(rendered template)

#### Errors

- In case of missing keys or unknown template returns 400.

## Hook API

It is possible to configure in the environment the key `RENDER_HOOK` to an URL which will receive the rendered template as parameter as follows:

```json
{
  "body": "RENDERED TEMPLATE HTML SYNTAX BASE64 ENCODED",
  "metadata": {
    /* data passed as hookMetadata to /render */
  }
}
```

---

## Support the developer

If you like tokyo-draft and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Contributing and issues

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve pavao, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog

View tokyo-draft's changelog [HERE](CHANGELOG.md)

---

## License

tokyo-draft is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
