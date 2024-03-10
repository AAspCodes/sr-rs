# sr-rs
tui based search and replace built with rust



### Uses
[ratatui](https://github.com/ratatui-org/ratatui/tree/main)
[tui-input](https://github.com/sayanarijit/tui-input)

## Todo

- [x] basic layout
- [ ] add file path box
- [ ] make text not disappear after pressing enter
- [ ] add clear keybind in normal mode
    - [ ] clear input boxes
- [ ] add search keybind in normal mode
- [ ] search for text matching pattern in current directory
- [ ] search based on file pattern
- [ ] show changes based on replaced
- [ ] implement file edit on replace
- [ ] improve vim integration
    - [ ] should show error if file has unsaved changes open in vim
    - [ ] should refresh vim
- [ ] replace all on replace list
- [ ] replace one at a time
- [ ] remove some from replace list
- [ ] hide some help text behind ? keybind
- [ ] cycle through matches/replace list, and use y/n/a/q, to replace, skip, replace all, quit
- [ ] add tests
- [ ] add perf benchmarks
- [ ] improve comments
- [ ] add links to imported project dependecies


## Quality of Life Todo
- [ ] precommit hook that runs cargo fmt
- [ ] implement basic CI pipline
- [ ] add some tool to automatically bump semver based on commit msg
- [ ] add linter
    - [ ] rust
    - [ ] markdown

## Strech Goals Todos
- [ ] publish binary
- [ ] add install script
- [ ] configurable styling
- [ ] integrate [edtui](https://github.com/preiter93/edtui) once it's mature enough
- [ ] add gif/video of example use
- [ ] add badges to README.md
    - [ ] dependencies
    - [ ] test coverage
    - [ ] build passing?
    - [ ] license
