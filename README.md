# CLI for Ever Platforms

[![Join the community on Spectrum](https://withspectrum.github.io/badge/badge.svg)](https://spectrum.chat/ever)
[![Gitter](https://badges.gitter.im/JoinChat.svg)](https://gitter.im/ever-co/ever?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Get help on Codementor](https://cdn.codementor.io/badges/get_help_github.svg)](https://www.codementor.io/evereq?utm_source=github&utm_medium=button&utm_term=evereq&utm_campaign=github)

A command line tool for Ever Platforms:

- https://github.com/ever-co/ever-demand
- https://github.com/ever-co/ever-gauzy

# Installation

```
npm install -g ever-cli
```
or
```
yarn global add ever-cli
```

# How to use

$ ever

# Publishing on NPM

## Using `np`

`npm run release`

After you run command above:
- answer a few questions (internally it runs [np](https://github.com/sindresorhus/np)) and it will automatically bump version & create new release draft on Github. 
- next CircleCI will automatically push new version to `npm` registry.

## Manually

To publish new release on [NPM registry](https://www.npmjs.com/package/ever-cli):
- bump package version number in the package.json file, _version_ field
- navigate to [releases page on Github](https://github.com/ever-co/ever-cli/releases), click "Draft a new release" and use version "v0.x.x".
- next CircleCI will automatically push new version to `npm` registry.

_Note: only versions starting with "v" will be published on npmjs.com_

## Collaborators and Contributors

### Development Team

#### Core

-   Ruslan Konviser ([Evereq](https://github.com/evereq))
-   Michael Konviser ([Komish](https://github.com/MrKomish))

#### Developers

-   Alish Meklyov ([Alish](https://github.com/AlishMekliov931))
-   Blagovest Gerov ([BlagovestGerov](https://github.com/BlagovestGerov))
-   Boyan Stanchev ([boyanstanchev](https://github.com/boyanstanchev))
-   Elvis Arabadjiyski ([Dreemsuncho](https://github.com/Dreemsuncho))
-   Emil Momchilov ([jew-er](https://github.com/jew-er))

#### Graphic Designer & QA

-   Julia Konviser

### Contributors

-   View all of our [contributors](https://github.com/ever-co/ever/graphs/contributors)

## Contact Us

-   [Spectrum Community](https://spectrum.chat/ever)
-   [Gitter Chat](https://gitter.im/ever-co/ever)
-   [Discord Chat](https://discord.gg/msqRJ4w)
-   [CodeMentor](https://www.codementor.io/evereq)
-   [Telegram](https://t.me/everplatform)
-   For business inquiries: <mailto:ever@ever.co>
-   Please report security vulnerabilities to <mailto:security@ever.co>
-   [Ever Platform @ Twitter](https://twitter.com/everplatform)
-   [Ever Platform @ Facebook](https://www.facebook.com/everplatform)

# Privacy & Tracking

We are using [Segment](https://segment.com) (via [analytics-node package](https://github.com/segmentio/analytics-node)) to monitor usage data of Ever CLI according to our [Privacy Policy](https://ever.co/privacy/apps).

## License

This software is available under different licenses

### _Ever Platform Community Edition_ License for CLI

If you decide to choose the Ever Platform Community Edition License for CLI, you must comply with the following terms:

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License, version 3,
as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.

[GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.txt)

### _Ever Platform Enterprise_ License

Alternatively, commercial versions of the software must be used in accordance with the terms and conditions of separate written agreement between you and Ever Co. LTD.

For more information about Ever Platform Enterprise License please contact <mailto:ever@ever.co>.

#### The default Ever Platform license, without a valid Ever Platform Enterprise License agreement, is the Ever Platform Community Edition License.

#### Please see [LICENSE.md](LICENSE.md) for more information on licenses.

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fever-co%2Fever-cli.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fever-co%2Fever-cli?ref=badge_large)

## Credits

Please see [CREDITS.md](CREDITS.md) file for a list of libraries and software included in this program and information about licenses.

## Trademarks

**Ever**® is a registered trademark of [Ever Co. LTD](https://ever.co).

The trademark may only be used with the written permission of Ever Co. LTD. and may not be used to promote or otherwise market competitive products or services.

All other brand and product names are trademarks, registered trademarks or service marks of their respective holders.

#### Copyright © 2019-present, Ever Co. LTD. All rights reserved.

[![npm downloads](https://img.shields.io/npm/dm/ever-cli.svg?style=flat)](http://npm-stat.com/charts.html?package=ever-cli)
[![Circle CI](https://circleci.com/gh/ever-co/ever.svg?style=svg)](https://circleci.com/gh/ever-co/ever-cli)
[![codecov](https://codecov.io/gh/ever-co/ever-cli/branch/master/graph/badge.svg)](https://codecov.io/gh/ever-co/ever-cli)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a5730f7dc949496faa3912ea8d31c022)](https://www.codacy.com/app/Ever/ever-cli?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=ever-co/ever-cli&amp;utm_campaign=Badge_Grade)
[![DeepScan grade](https://deepscan.io/api/teams/3293/projects/4851/branches/38568/badge/grade.svg)](https://deepscan.io/dashboard#view=project&tid=3293&pid=4851&bid=38568)
[![Known Vulnerabilities](https://snyk.io/test/github/ever-co/ever-cli/badge.svg)](https://snyk.io/test/github/ever-co/ever-cli)
[![Greenkeeper badge](https://badges.greenkeeper.io/ever-co/ever-cli.svg)](https://greenkeeper.io/)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fever-co%2Fever-cli.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fever-co%2Fever-cli?ref=badge_shield)
