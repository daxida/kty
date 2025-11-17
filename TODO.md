- [ ] try to pass paths instead of the whole pm

- [x] path manager
- [x] glossary dict should be output in el/en/dict but the temp files in el/en/temp_glossary 
- [x] add sense translation to see if they change tests > they dont
- [x] gloss dict testing
- [x] gloss dict working (the formatted ones)
- [x] gloss dictionaries
- [x] update README
- [x] gloss dict basic working (did Text)
- [x] filter when the edition is english

## FAILED
- also pass cached filtering wordentries
  Turned out to be slower, not sure why
- test deserialize to &str
  Failed because it requires BIG ASSUMPTIONS on the characters (f.e. that it does not have to escape stuff)

## USELESS BACKLOG
- [ ] I don't think the build.py is really needed, maybe just read the jsons at runtime...
- [ ] dont hardcode forms/lemmas when writing IR < write_tidy_result (apparently this is done by original for forms only?)
- [ ] Be faster ? flamegraph (A way to be faster is to shrink as much as possible the Tidy objects)
- [ ] localize tags for fun? pointless, kaikki already took this decision of using English as lingua franca
- [ ] Exit code? pointless
- [ ] calver? maybe if I ever do releases
- [ ] fix subcommands (shared args) > is this even possible?

## DIFFS
- filetree
- made DATA deletable (important assets MUST be somewhere else)
- Do not use raw_glosses/raw_tags
- Fixed merge_person_tags not merging three persons at once
- Fixed etymologies being added in the wrong order (αρσενικό)
- Fixed etymologies missing
- (Potentially) add final dots
- sorting order when serializing
- sorting order for tags in forms term_bank (the original didn't sort which caused duplicates and inconsistent order)
- dont download gz for En edition
- dont extract for En edition
- pass tidy IR result when possible
- deinflections are wrongly serialized in Tidy
- Japanese dict is broken
- Added FormSource for better debugging
- The thai (th) testsuite is about a malformed page > ignore

## NOTES
- the '\r' trick depends on terminal size!
- [x] ureq over reqwest - bloaty-metafile 
      @ [reddit](https://www.reddit.com/r/rust/comments/1osdnzd/i_shrunk_my_rust_binary_from_11mb_to_45mb_with/)
