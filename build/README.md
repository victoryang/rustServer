
## Change repo to ustc
### modify ~/.cargo/config
	[source.crates-io]
	replace-with = 'ustc'

	[source.ustc]
	registry = "git://mirrors.ustc.edu.cn/crates.io-index"

## cargo update
### To update crates