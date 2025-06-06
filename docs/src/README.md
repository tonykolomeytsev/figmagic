# Introduction

FigX is a CLI tool for managing assets exported from Figma. It's an ideal choice for medium to large projects, as it doesn’t require changes to your project structure and is simple to configure.

- ⚡ **Fast**: FigX exports and transforms assets from Figma significantly faster than figma-export, and orders of magnitude faster than doing it manually — the only speed limit here is Figma’s REST API rate limit 😅

- ⚖️ **Correct**: The result of importing assets into your project precisely reflects what’s defined in the FigX configuration — no more, no less.

- 🛡️ **Reliable**: Importing assets through FigX yields the same result on any machine and any OS. Non-reproducible results can only occur due to changes made by designers in Figma, not because of the developers or the tool.

- ♻️ **Caching**: Network requests are minimized. Only changed assets are fetched. The cache is portable across machines, making it perfect for CI pipelines and automated workflows.
