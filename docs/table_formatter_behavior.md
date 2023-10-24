# Table Formatter Behavior

## Extra Columns

Any GFM-compliant renderer will only render the number of columns included in the delimiter row.  Any additional columns in the source are ignored.  For example,

```markdown
| 1    | 2    | 3    |
| :--- | :--- | :--- |
| 1    | 2    | 3    | 4|
| 1    | 2    | 3    |44|
```

renders to

| 1    | 2    | 3    |
| :--- | :--- | :--- |
| 1    | 2    | 3    | 4 |
| 1    | 2    | 3    | 44 |

The formatter **does not** delete any non-whitespace characters.

The formatter will include any additional columns, but it will not attempt to align or standardize width of any extra content.  Since the formatter doesn't have any information about column max width, it sets whitespace to a single space at the beginning and end of each cell.  This means that the extra columns may not align properly.  For example,

```markdown
| 1    | 2    | 3    |
| :--- | :--- | :--- |
| 1    | 2    | 3    | 4 |
| 1    | 2    | 3    | 44 |
```

The formatter attempts to be as non-intrusive as possible, so it does not alert the user that the above Markdown table is improperly constructed.

## Too Few Columns

If a row has fewer columns than the delimiter row specifies, then no additional columns are added.  The formatting of the below table will not change.

```markdown
| 1    | 2    | 3    |
| :--- | :--- | :--- |
| 1    | 2    |
| 1    | 2    | 3    |
```

A trailing vertical bar will be added if one does not exist.  For example, formatting the table below will result in the table above.

```markdown
| 1    | 2    | 3    |
| :--- | :--- | :--- |
| 1    | 2
| 1    | 2    | 3    |
```

## Double Width Characters

Markdown Table Formatter's sister crate [unicode-display-width](https://github.com/jameslanska/unicode-display-width) is used to determine the display width of each table cell string.  Refer to its [documentation](https://github.com/jameslanska/unicode-display-width/tree/main/docs) for details.
