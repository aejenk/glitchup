# Self Notes

Some notes for myself to keep to. When I'll be working, I'll try to follow
these.

## Regarding the CHANGELOG

After some time, I decided to add a changelog for `glitchup`. I feel as if it
would help keep note of what changed. This can be done manually, but I wanted
to try and use [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/). On one hand,
it helps make commits clearer, specifically with respect to what they're trying
to do. ~~On the other hand, I'm using
[`jilu`](https://github.com/rustic-games/jilu#%E8%AE%B0%E5%BD%95) to generate
the changelog.~~ Trying to install `jilu` breaks, meaning its use is impossible.
As a result, the changelog has been changed to be more customized, and `jilu` will 
no longer be used.

An attempt to keep commits conventional however, will still be made.

### Conventional Commits

The following serves as a reminder for how to conventionally commit:

- **feat:** a new feature
- **fix:** a bugfix
- **docs:** documentation only changes
- **style:** changes that don't affect the meaning of the code
- **refactor:** code change that neither fixes a bug, nor adds a feature
- **perf:** code change that improves performance
- **test:** adding missing tests
- **chore:** changes to build process / auxiliary tools / libraries such as
  documentation generation
