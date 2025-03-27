<!-- markdownlint-disable -->
<div align="center">
<h1>Rust Tooling</h1>

[![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)][github]
[![GitHub Stars](https://img.shields.io/github/stars/42ByteLabs/rust-tooling?style=for-the-badge)][github]
[![GitHub Issues](https://img.shields.io/github/issues/42ByteLabs/rust-tooling?style=for-the-badge)][github-issues]
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)][license]

</div>
<!-- markdownlint-restore -->

The [Rust-Tooling][github] project is a collection of tools that can be used to help with the development of [Rust][rust-lang] projects in GitHub Actions.

## ✨ Features


### 🚀 Publishing

```yaml
- name: Publish to Crates.io
  uses: 42ByteLabs/rust-tooling/publish@v0.1.5
  with:
    token: ${{ secrets.CARGO_PUBLISH_TOKEN }}
```

## 🧑‍🤝‍🧑 Maintainers / Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://geekmasher.dev"><img src="https://avatars.githubusercontent.com/u/2772944?v=4?s=100" width="100px;" alt="Mathew Payne"/><br /><sub><b>Mathew Payne</b></sub></a><br /><a href="#code-GeekMasher" title="Code">💻</a> <a href="#review-GeekMasher" title="Reviewed Pull Requests">👀</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## 🦸 Support

Please create [GitHub Issues][github-issues] if there are bugs or feature requests.

This project uses [Semantic Versioning (v2)][semver] and with major releases, breaking changes will occur.

## 📓 License

This project is licensed under the terms of the MIT open source license.
Please refer to [MIT][license] for the full terms.

<!-- Resources -->

[license]: ./LICENSE
[github]: https://github.com/42ByteLabs/rust-tooling
[github-issues]: https://github.com/42ByteLabs/rust-tooling/issues
[rust-lang]: https://www.rust-lang.org/
[semver]: https://semver.org/

