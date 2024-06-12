# url dispatcher

```bash
# build
cargo build --release

# register
install.bat install UrlDispatcher "\"url-dispatcher.exe\" \"%1\"" "url-dispatcher.exe,0"

# set as default browser
SetDefaultBrowser.exe HKCU UrlDispatcher

# uninstall
install.bat uninstall UrlDispatcher
```

[How to set program.exe as a default browser in Windows 10?][1]

[1]: https://stackoverflow.com/questions/46174517/how-to-set-program-exe-as-a-default-browser-in-windows-10
