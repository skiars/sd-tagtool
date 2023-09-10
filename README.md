# sd-tagtool

[中文看这里](README-CN.md)

This is a simple tag editor for stable diffusion datasets. It can be used to edit datasets generated by automatic labeling tools. This tool is inspired by [BooruDatasetTagManager](https://github.com/starik222/BooruDatasetTagManager).

### Features

- Support display and management of all image tags
- Support undo/redo
- Intelligent prompt for tag input (support fuzzy matching)
- Support drag and drop for tags
- Tags can be inserted/deleted in batches, and the insertion position can be specified
- Datasets can be filtered by tags
- Customized tag highlighting
- Automatic translation (translation to Chinese is now hard-coded, and may require magic to surf the Internet)
- Quick response

## Screenshot

![screenshot.png](images/screenshot.webp)

## Download / Install

sd-tagtool supports Windows, macOS, and Linux. The installation steps are as follows:
1. Find the latest version on the [Release](https://github.com/skiars/sd-tagtool/releases) page;
2. Download the installation package file from the **Assets** list according to the OS, for example, the suffix of the Windows installation file is **_.msi_** or **_.exe_**;
3. Run the installation file.

**Note**: sd-tagtool may require newer Windows 10 or Windows 11. Also, I don't often test for compatibility with Linux and macOS.

## Usage

You can try the basic usage by yourself, only a few details are added here.

### Basic tag editing

When you select a picture, drag the tag of the image to sort it, or click the `×` icon on the tag to delete. But the tags cannot be sorted when multiple images are selected.

You can also turn on the *edit all tags* switch, which will:
- Delete tags in the entire data set (all tags are displayed at the bottom of the window). For example, deleting "1girl" in all tags will cause the "1girl" tag in all images to be deleted;
- When inserting or replacing tags, all images in the dataset will be operated, not just the selected images.

### Tag highlight

You can choose a highlight color for the tag in *Pick color* of the tag's context menu (click the right mouse button), so that you can find it quickly. When you don't need the tag highlighting anymore, you can clear it by *Clear picked color* in the context menu.

### Select tags

Click a tag with the left mouse button in the tag list to select it. Hold down the `Ctrl` key to select multiple tags, and hold down `Shift` to select multiple tags by range. The selected tags can be copied or added to the filter through the content menu.

### Tags filter

Enter the tags to be filtered in the tags filter input at the top, and then click the *Filter* button to filter the dataset. Two filtering modes can be selected via the *exclude* checkbox:
- **Include mode**: when the image has all the tags in the filter, it will be displayed in the filtered list;
- **Exclude mode**: Images that do not have all the tags in the filter will be shown in the filtered list.

After editing the dataset, you need to click the *Filter* button again to update the filtered dataset. You can enter tags manually, or right-click in the tags list and add selected tags to the filter via the *Add filter* menu.

### Insert tags

Enter a tag in the tags input box and click the *Insert* button to insert a new tag into the selected dataset (you can select multiple images). As you can see tags input box can fill in multiple tags.

The insertion position is specified by the *position* box. These modes are currently supported:
- **auto**: Insert tags to tail if there is no label to be inserted in the image, otherwise do nothing;
- **Positive number**: Insert tags into the position counting from the head, if the tag already exists in the image, it will be moved to the specified position;
- **Negative number**: Similar to positive numbers, but counting from the tail to the front.

The insertion position can exceed the actual number of tags in the image, and the tags will be inserted at the head or tail position at this time.

Double-clicking a tag directly in the list of all labels will also insert it into the tag set of the selected images. The insertion position at this time is also determined by the *position*.

### Replace tags

Click the `>` button on the left side of the tag editing bar to open the tag replacement bar. Enter the tags for replacement in the *replace with* input box, and click the *Replace* button to replace the tags. Tag replacement is one-to-one correspondence, for example:
- Replace `a,b,c` with `d,e,f` means: `a` is replaced by `d`, `b` is replaced by `e`, `c` is replaced by `f`;
- Replace `a,b,c` with `d,e` means: `a` is replaced with `d`, `b` is replaced with `e`, `f` is deleted;
- Replace `a,b` with `d,e,f` means: `a` replaced with `d` and `b` replaced with `e,f`.

The specific replacement process for each tag is:
- The replaced tag will appear in the position of the original tag;
- If the replaced tag already exists in the image tag list, subsequent duplicates will be removed;
- If the image does not have the original tag, the corresponding replacement will not be performed;
- You can also delete tags by replacing them with nothing.

### Undo / Redo

Click *Undo* and *Redo* in the *Edit* menu to undo and redo, and you can also use the shortcut keys to do it.

There is no step limit for undo and redo, but there is currently no reasonable interactive feedback (this will lead to you may not know what happened). Also, the undo history is cleared after opening a new directory.

### Translation

Click the *Translate tags* menu of *View* to enable automatic translation.

## Warning

Don't press Ctrl + R!!! This will refresh the page and lose all data.

## Development

This project is based on [Tauri](https://tauri.app/) and [Vue.js](https://vuejs.org/). To build this project, you need to install Nojde.js and pnpm. Use these commands to install to Windows:
``` bash
winget install nodejs # install Node.js
winget install pnpm   # install pnpm
```
Then follow this [document](https://tauri.app/v1/guides/getting-started/prerequisites) to configure the Rust toolchain and WebView2. You can also install rustup with winget, if you use Windows:
``` bash
winget install rustup
```

Use the following commands to build when everything is in place:
``` bash
pnpm install   # install all dependencies
pnpm tauri dev # build the debug program and start
```
According to my test, `pnpm tauri dev` is abnormal under Linux desktop, and can only be debugged with Windows or macOS.

Build the release version with the these command:
``` bash
pnpm tauri build
```

The user-facing installation package is automatically built by GitHub Action, and the specific configuration is [here](.github/workflows).
