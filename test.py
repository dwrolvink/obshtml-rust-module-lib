"""
Make sure to first run `pip install obsidianhtml`.
See also the readme

This script will run your module like obsidianhtml will, but with a dummy module data folder.
Change this path if you are on a system that doesn't have /tmp, like Windows.
"""

from obsidianhtml_rust_module_example.module import ObsidianHtmlRustExampleModule
module = ObsidianHtmlRustExampleModule(module_data_fpps="dummy", module_name="rust_test")
module.run()