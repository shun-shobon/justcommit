# Changelog

## 0.1.0 (2023-08-14)


### Features

* Add `generateCommitMessage()` ([cb7bb0f](https://github.com/shun-shobon/justcommit/commit/cb7bb0f2ccff1549c15bfab0924a0ac1b4c19016))
* Add `readConfig()` and dependencies ([617e67e](https://github.com/shun-shobon/justcommit/commit/617e67e039ffd55c6f3f1a615cbb7a2ad0039786))
* Add `readToken()` ([c2fe20b](https://github.com/shun-shobon/justcommit/commit/c2fe20b9ac086582a96d49b58be61beee7b3f873))
* Add a sample of making requests to OpenAI ([262de5e](https://github.com/shun-shobon/justcommit/commit/262de5e4d5d4454dfcc3ac752abe3d851b325327))
* Add ChatGPT ([f05707c](https://github.com/shun-shobon/justcommit/commit/f05707caa489970e1415206e86877bc0a7a52f99))
* Add command parser ([634f4ea](https://github.com/shun-shobon/justcommit/commit/634f4ea3861fe7cfa61cd59f65154541a7bce493))
* Add commandline args parser ([a65c111](https://github.com/shun-shobon/justcommit/commit/a65c11176c5e8c0a969dba47927c097d47e48bb7))
* Add Git command runner ([63d15aa](https://github.com/shun-shobon/justcommit/commit/63d15aacf650111b2422b89aff6a291c782ee6c4))
* Add git2 ([65ebf2a](https://github.com/shun-shobon/justcommit/commit/65ebf2a28b7fc5a6b165f7af75686a5ab80ec5f0))
* Add ignore pattern ([7e28ae1](https://github.com/shun-shobon/justcommit/commit/7e28ae1fe2e90a1882de261c6e79386c79651a8c))
* Add OpenAI client ([be30c82](https://github.com/shun-shobon/justcommit/commit/be30c82eac86add12e35fde49a1f5a12c2a72e75))
* Add parameter to generator to output more easy to understand text ([4ff8f2d](https://github.com/shun-shobon/justcommit/commit/4ff8f2de7ecc36c07c0230d072f36dfd40b6025b))
* **config.rs:** Implement fetching token from 1Password using 'op' CLI command ([98f2878](https://github.com/shun-shobon/justcommit/commit/98f287828eb229f5745a0306e8111c23383b7563))
* **config.rs:** Update `RawConfig` struct to use `OpenAIToken` enum for token type and value ([c2fdc5b](https://github.com/shun-shobon/justcommit/commit/c2fdc5bda2f8288ca0e4435dbc570a03b1fe37cd))
* **config:** Add config module to handle loading of application configuration ([00dbf42](https://github.com/shun-shobon/justcommit/commit/00dbf42a9d7a7fa120d2269f81e9b1373d9ea1f0))
* Ensure changed file is not empty ([3a75215](https://github.com/shun-shobon/justcommit/commit/3a7521533ea2f10941be526761c5d59724514ec0))
* Initial commit ([4509282](https://github.com/shun-shobon/justcommit/commit/4509282d6a5f099735b34b3a50b2717a7c09536c))
* Initialize rust ([d8c81bc](https://github.com/shun-shobon/justcommit/commit/d8c81bc58913a837fd2725391e0e86582220a5af))
* **main.rs:** Implement chat-based commit message generation using ChatGPT ([c1c23b1](https://github.com/shun-shobon/justcommit/commit/c1c23b193e15bede67852440f0544a98b95acfa6))


### Bug Fixes

* **config.rs:** Fix getting config dir ([b559e3a](https://github.com/shun-shobon/justcommit/commit/b559e3aa540d59c8eafd870c87d1f3f3aec817b5))
* Fix by clippy ([9c86651](https://github.com/shun-shobon/justcommit/commit/9c866510d51c75596a568a00588884ff5c570088))
* Remove unnecessary context ([edb0acd](https://github.com/shun-shobon/justcommit/commit/edb0acd29bc3c57abdbf1cdcc60707029f7e454b))
* Update commit message constraints ([352a685](https://github.com/shun-shobon/justcommit/commit/352a6851e85be9208c620486cf0623c2960acab1))
