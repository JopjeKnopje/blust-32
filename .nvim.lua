require("lspconfig").rust_analyzer.setup({
	settings = {
		["rust-analyzer"] = {
			check = {
				allTargets = false,
			},
			cargo = {
				target = "thumbv7m-none-eabi",
			},
		},
	},
})
