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
Note that there's a `makefile` that will let you call `make` to run this and the two `crate runs` below.
* Run the Rust program to get the list of every journal in Springer's alphabetical index:
  `crate run get_jrnls`.
* This generates a file named `urls`.
* Run the Rust program to click every journal in Springer's alphabetical list and annotate whether
  certain policies are included: `crate run get_tab`. This generates a file named `tab`.
* Read the output text into an SQLite database using your favorite method. I use `apop_text_to_db` from the Apophenia project. The database file should be named `t.db` and the table `t`.
* Run the `list_fields` script to get the final per-field tallies.

Policies
======

A large number of the journals on Springer's index are defunct and archived.
I skipped over these entirely; they are not counted at all.

I get the impression that editors are given sets of boilerplate code to choose from and select the one that best fits their needs.
For example, Springer has four tiers of research data policy, and many journal policies refer to those.

To give an example of a key passage we're looking for:

> Submission ...  implies that materials described in the manuscript, including all relevant raw data, will be freely available to any scientist wishing to use them for non-commercial purposes, without breaching participant confidentiality.

See the code itself for the exact snippet I looked for. I look for other biolerplate, or explicit references to the data-publication-requiring tiers 3 and 4.

Many journals point to what seems to be the default Springer policy
https://www.springeropen.com/get-published/editorial-policies
which "strongly encourages" data publication.
There are many other variants of "authors should" or "authors are strongly
encouraged to".

On my last run, 14% of journals aren't categorized by the script as either requiring or only encouraging publication, typically because the editors posted a PDF with requirements or data isn't mentioned at all.
I hand-checked a lot of these and most were low-tech journals (like literature journals), and I couldn't find a one who had a data publication requirement buried in there somewhere.
For one or two that I checked, the primary technical demand seemed to be that authors are encouraged to submit electronically instead of mailing the paper to the editors.

A small number of journals are operated by a professional society but are administered by Springer, and the society requirements are posted on a separate web site and are a little more likely to have a data requirement.
However, any given field has only one or two such societies, and I'm aggregating to broad fields with over a hundred journals.
I'm comfortable saying that not being able to classify this small number of journals makes the results potentially imprecise but not inaccurate enough to invalidate the qualitative lessons or cause serious distortions across fields.
