# materials_for_replication

This repository provides the code used to generate the table in [name/link to article] giving the percent of journals in different fields with an open data/code requirement.

It goes through Springer's index of journals and opens the editorial requirements page for each, as one would do in one's browser.  It doesn't do anything described in common language as "downloading", such as pulling PDFs of journal articles.

In the article, Josh Greenberg says that researchers are reluctant to release their code:
"Whenever they manage to find the week to really clean up the code, they'll be happy to release it, but that week is never available."

Yes. This project was an excuse to learn to use Rust, and still has some hacks, mentioned below.
The page reads are run serially, with the WebDriver closed and reopened between every
journal, making the process slow but very reliable.

Overview:

* Start a docker instance with the appropriate version of Selenium. Paste this onto your command line:
  
```
docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-firefox-debug:3.141.59-zinc
```

* Run the Rust program to get the list of every journal in Springer's alphabetical index pages.
* Run it again after changing the `1` on line 34 to a `2` to get the page twos of the alphabetical
  index. See above for the apology about hacks.
* This generates a file named `urls`.
* Run the Rust program to click every journal in Springer's alphabetical list and annotate whether
  certain policies are included. This generates a file named `tab`
* Read the output text into an SQLite database using your favorite method. I use `apop_text_to_db` from the Apophenia project. The database file should be named `t.db` and the table `t`.
* Run the `list_fields` script to get the final per-field tallies.

Policies
======

A large number of the journals on Springer's index are defunct and archived.
I skipped over these entirely; they are not counted at all.

Journal policies have many elements in common. I get the impression that
editors are given sets of boilerplate code to choose from and select the one
that best fits their needs.
For example, Springer has four tiers of research data policy, with boilerplate descriptions.

Here is the key passage we're looking for:

Submission ...  implies that materials described in
the manuscript, including all relevant raw data, will be freely available to any scientist
wishing to use them for non-commercial purposes, without breaching participant confidentiality.

My run found 477 journals using this text.
See the code itself for the exact snippet I looked for.

A small number (5) had boilerplate including:
"program code and model parameter data sets must be made freely available for academic use, and
requires that authors abide by this principle."

Many journals point to what seems to be the default Springer policy
https://www.springeropen.com/get-published/editorial-policies
which "strongly encourages" data publication.
There are many other variants of "authors should" or "authors are strongly
encouraged to".
The stats for these aren't used and aren't discussed in the article.
There are certainly many other ways of asking nicely that the program doesn't consider.

That's all the verbiage about requiring data publication that I could find after hand-checking
a large number of policy pages.

A small number of journals are operated by a professional society but are
administered by Springer, and the editorial requirements are written by the
society and posted on their own web site.
I can't parse those pages.
However, any given field has only one or two such societies, and I'm
aggregating to broad fields with over a hundred journals.
These society journals therefore make the results imprecise, but can not shift the
overall tallies by more than a few points one way or the other.
[If this were an academic paper with more space for caveats, we could say that
the study population is journals sufficiently controlled by Springer that
their web presence is entirely hosted by Springer itself.]


TL;DR: It is almost certain that I missed some number of journals with data
policies with distinct wording, meaning that the counts in the article are
underestimates.
I'm confident that the level of underestimate is not large enough to change
the qualitative results, and is not so concentrated in one field that the
results are especially distortionary.
