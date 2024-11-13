# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 0.6.3 - 2024-11-13
#### Bug Fixes
- update static images urls - (0342b68) - aeyoll

- - -

## 0.6.2 - 2024-11-12
#### Continuous Integration
- **(perf)** add cache to rust dependencies - (a865bcc) - *aeyoll*
- update release name - (ac30388) - aeyoll

- - -

## 0.6.1 - 2024-11-12
#### Bug Fixes
- **(cli)** trim content before saving - (c5deef2) - *aeyoll*
- **(cli)** print an error when no redirection or indirection is provided - (5757eb4) - *aeyoll*
- **(web)** trim content before saving - (ca3f392) - *aeyoll*
#### Build system
- add cli to the build script - (4e5b0d6) - aeyoll
#### Continuous Integration
- add a script to build on github - (b0bc664) - aeyoll
#### Documentation
- fix a typo - (3d32a73) - aeyoll
- remove nodejs from the requirements - (cba1b49) - aeyoll
#### Miscellaneous Chores
- get api url environment variable at runtime - (23323ed) - aeyoll

- - -

## 0.6.0 - 2024-11-02
#### Build system
- **(cli)** set api url at compile time - (be322bd) - *aeyoll*
- upgrade deps - (3f07c6e) - aeyoll
- add cli build script - (03d19fc) - aeyoll
- use trunk compilation for tailwind - (784862a) - aeyoll
#### Documentation
- add cli instructions - (4f85d4f) - aeyoll
#### Features
- allow to add a paste from cli - (04b05c0) - aeyoll
- add icons in the navigation - (af99d96) - aeyoll
- move the home page to an about page - (94cdea9) - aeyoll
#### Refactoring
- move the encryption functions to a separate crate - (7991ac3) - aeyoll

- - -

## 0.5.1 - 2024-10-31
#### Bug Fixes
- break spaces in pre elements - (329f8b8) - aeyoll
#### Build system
- add a makefile - (54335b6) - aeyoll
#### Documentation
- add a sample nginx configuration in the README - (d11e24a) - aeyoll
#### Features
- display current version in the navbar - (ca3f9c8) - aeyoll
#### Style
- reindent css - (5bba23a) - aeyoll

- - -

## 0.5.0 - 2024-10-22
#### Bug Fixes
- fix router - (63a6961) - aeyoll
#### Build system
- upgrade sea-orm - (1f98b21) - aeyoll
#### Documentation
- update README - (d4c46f6) - aeyoll
#### Features
- add text when copying to clipboard - (25f767c) - aeyoll
- use anchor to fetch the key - (b04fb87) - aeyoll
- add homepage content - (796264b) - aeyoll
- add dark mode - (a6f42f8) - aeyoll
- improve styling - (2230ce6) - aeyoll
- allow to copy and download a paste - (0469c0b) - aeyoll
- add a logo - (4b3dff8) - aeyoll
- add syntax highlight in the frontend - (ba26263) - aeyoll
- allow to filter log messages using compile time env variable - (5307c21) - aeyoll
- add default api url env variable - (914b185) - aeyoll
- finish to implement decoding - (6e20996) - aeyoll
- start to implement decoding - (2b86a81) - aeyoll
- add redirection to the paste page - (643d024) - aeyoll
- add encryption methods - (9f5a007) - aeyoll
- set actions - (4c2e003) - aeyoll
- add new paste form - (b820848) - aeyoll
- add tailwind postcss conf - (0da5f63) - aeyoll
- add an active route component - (3e0133d) - aeyoll
- add link to home - (ab51160) - aeyoll
- add 404 integration - (6a0fccf) - aeyoll
- start integration - (9bc6f01) - aeyoll
- add basic pages - (6b67fef) - aeyoll
#### Miscellaneous Chores
- remove static assets - (a3d0759) - aeyoll
- cleanup dependencies - (b967522) - aeyoll
- remove unused options and crates - (8cd0f91) - aeyoll
- wording - (5ee2238) - aeyoll
- delete an unused route - (753a7a8) - aeyoll
- rename title - (d664667) - aeyoll
- fix clippy errors - (1d8e090) - aeyoll
- remove logger - (c140ef3) - aeyoll
- delete yarn.lock file - (88d418d) - aeyoll
- add missing body to app - (4b86a97) - aeyoll
- updated lock files - (8d339a9) - aeyoll
- update rust dependencies - (a5efc8b) - aeyoll
- update js dependencies - (969fcee) - aeyoll
#### Refactoring
- add a prefix to the api routes - (3130cef) - aeyoll
- remove unused routes - (1805760) - aeyoll
- move profile configuration to the workspace - (98f367c) - aeyoll
- remove index from api - (ec6af99) - aeyoll
- simplify display of highlight content - (3aa349a) - aeyoll
- remove unused field - (80fe9b2) - aeyoll
- simplify code - (8609fec) - aeyoll
- move to separate crates - (9cdf419) - aeyoll
#### Style
- fmt - (33ec634) - aeyoll

- - -

## 0.4.1 - 2022-12-04
#### Miscellaneous Chores
- update Rust dependencies - (2fcf36b) - aeyoll
- update js dependencies - (789f5c0) - aeyoll

- - -

## 0.4.0 - 2022-08-27
#### Features
- add an info message when there's no paste - (9ea2aaa) - aeyoll
#### Miscellaneous Chores
- update cog configuration - (1aca04b) - aeyoll
- code styling - (a5b3c89) - aeyoll
#### Refactoring
- add a helper function to render a template - (950723c) - aeyoll
- rename route create to create_paste - (9d7c507) - aeyoll
- convert everything to axum - (e29f908) - aeyoll

- - -

## 0.3.3 - 2021-12-13
#### Features
- add the "private" command line flag, to define if the paste form should check private or not (disabled by default) - (21c986a) - Jean-Philippe Bidegain
- - -

## 0.3.2 - 2021-12-09
#### Bug Fixes
- prevent crash when creating a private paste - (5393ae6) - Jean-Philippe Bidegain
- set default order in the homepage by create time - (25e23cb) - Jean-Philippe Bidegain
- - -

## 0.3.1 - 2021-12-09
#### Miscellaneous Chores
- run clippy - (4e75bd8) - Jean-Philippe Bidegain
- add cog pre and post bump hooks - (7af4afd) - Jean-Philippe Bidegain
- - -

## 0.3.0 - 2021-12-09
#### Features
- add a "private" field to the paste - (6326b7d) - Jean-Philippe Bidegain
- migrate the identifier from an int to a "nanoid" - (28c5aa6) - Jean-Philippe Bidegain
#### Miscellaneous Chores
- update cocogitto config - (c1d2ffd) - Jean-Philippe Bidegain
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
