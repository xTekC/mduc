# markdup
A markdown to html compiler.

## Run with Bash:
---
```
sh <(curl https://markdup.onrender.com/sh)
```

## Run with Powershell:
---
```
iwr -useb https://markdup.onrender.com/ps | iex
```

# Usage:
Within the directory of the markdown file, run: `markdup md_file.md` <br>
This will output an equivalent `.html` file.

<br>

# Capabilities :
- converts `#` to `<h1></h1>` tags
- converts `plain text` into `<p></p>` tags

<br>

# Future Additions :
- `<html>` html
- `<head>` head
- `<body>` body
- `<title>` title
- `<h2>`, `<h3>`, `<h4>`, `<h5>`, `<h6>` headers
- `<em>` emphasis
- `<b>` bold
- `<i>` italic
- `<small>` small text
- `<u>` underline
- `<strike>` strike through (deleted text)
- `<a href="">` anchor
- `<li>` list
- `<ol>` ordered list
- `<ul>` unordered list
- `<!-->` comment
- `<marquee>` scrolling text
- `<center>` center
- `<font>` font
- `<br>` line break
- `<img>` image
- `<link>` link
- `<hr/>` horizontal rule (display horizontal line)
- `<meta>` meta (page description)
- `<table>` table
- `<tr>` table row
- `<th>` table header
- `<td>` standard table cell
- `<form>` form
- `<input type="submit">` submit input
- `<option>` dropdown option
- `<input type="radio">` radio input