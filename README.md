## VSCode Git Bridge

解决在VSCode中无法使用MSYS2 git问题

相关问题：
[https://github.com/microsoft/vscode/issues/4651](https://github.com/microsoft/vscode/issues/4651)

但是由于批处理无法处理参数中带有`^`等转义字符参数导致`vscode-git-graph`无法使用，所以使用Rust简单实现了下。