# Turn the donor-journey recording back on

Turn the donor-journey recording back on. journey.mp4 and journey-poster.jpg in site/static/images have been replaced with a version without the security claim (its last panel now says "All three agree").
- In site/templates/partials/journey.html set show_recording = true and remove the comment above it.
- In site/zola.toml remove the ignored_static line for the two journey files and its comment.
- In site/content/work/donation-website.md remove the Nows comment about the recording.
- Check the recording block has no security claim in its text or caption.
Keep draft = true. Leave the order uncommitted.
