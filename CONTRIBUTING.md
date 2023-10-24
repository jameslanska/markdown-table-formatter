# Contributing to Markdown Table Formatter

First off, thank you for considering contributing to the Markdown Table Formatter.  This document will help answer common questions you may have during your first contribution.

## Issue Reporting

Not every contribution comes in the form of code. Submitting, confirming, and triaging issues is an important task for any project.

We use GitHub to track all project issues.  If you

- discover bugs,
- discover a incorrect width output,
- have ideas for improvements or new features, or
- notice a problem with the documentation,

please start by opening an issue on this repository.  We use issues to centralize the discussion and agree on a plan of action before spending time and effort writing code that might not get used.

If you find a security vulnerability, do ***NOT*** open an issue.  Follow the instructions in `SECURITY.md`.

### Submitting An Issue

1. Check that the issue has not already been reported.
2. Select the appropriate issue type, open an issue with a descriptive title, and follow the template.
3. Be clear, concise, and precise using grammatically correct, complete sentences in your summary of the problem.
4. Include any relevant Markdown input and Markdown output in the issue.

## Code Contributions

Table Formatter follows a [forking workflow](https://docs.github.com/en/get-started/quickstart/contributing-to-projects) with the following contribution process

1. Open an issue on the [project repository](https://github.com/jameslanska/markdown-table-formatter/issues), if appropriate.
2. Fork the project <https://github.com/jameslanska/markdown-table-formatter/fork>.
3. Create your feature branch `git checkout -b my-new-feature`.
4. Commit your changes git `commit -am 'Add some feature'`.
5. Push to the branch `git push origin my-new-feature`.
6. Create a [GitHub Pull Request](https://help.github.com/articles/about-pull-requests/) for your change following instructions in the pull request template.
7. Participate in a Code Review with the project maintainer on the pull request.

Releases will most likely follow quickly after merging into `main`.

### What We Are Looking For

- bug fixes
- documentation improvements
- code style improvements
- performance improvements
- etc.

### What We Are Not Looking For

We are not not looking for implementation of configuration choices.  This crate intentionally does not allow for configuration.  This minimizes the cognitive load on the end user and limits the bug surface for the project.

A pull request that only serves to add a configuration choice will almost certainly be rejected.

## Coding conventions

Start reading the code and you'll get the hang of it.  We optimize for readability:

- every change must have corresponding tests
- version numbers follow [Semantic Versioning](https://semver.org/)
