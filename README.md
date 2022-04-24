# tagr - cross-platform file tagging!

tagr is a very simple tool for organising your files into tags, as a supplement
to directories.

### Why this exists

Let's imagine that you're taking a class. Where do you store your lectures?

```
~/Documents/school/<course_id>/lectures/ ?
```
```
~/Videos/lectures/<course_id>/ ?
```
```
/mnt/drive/some_other_location/ ?
```

Ideally, you'd be able to use a combination of all 3, but as we're stuck with
tree-like directory structure, you'll always have to commit to *one.*

tagr aims to solve that problem. The goal is to provide an interface through
which you can manage *tags* for files on any platform. This can then be combined
with other UNIX-friendly tools, working similarly to programs like `ls`, `find`,
`mv`, and the like.

### Usage

Let's take the scenario above. Using tagr, you might solve this dilemma like so:

```bash
$ mv ./lecture.mp4 /mnt/disk/
$ tagr add lectures /mnt/disk/lecture.mp4
$ tagr add $COURSE_ID /mnt/disk/lecture.mp4
$ tagr add $COURSE_ID-media /mnt/disk/lecture.mp4
```

Now, the lecture is physically stored on your external disk, but can be accessed
in multiple ways:

1. Using `tagr ls lectures` (for all lectures)
2. Using `tagr ls $COURSE_ID` (for all course materials)
3. Using `tagr ls $COURSE_ID-media` (for all course-related media)

### Planned features

tagr is still in early development, and is currently very minimal. It's supposed
to stay that way, but there are some more features planned:

- Complex queries
    - Tag intersections
    - Tag differences
    - Tag unions
- Better scriptability
    - Finding individual files
        - eg. `tagr get $COURSE_ID/lecture.mp4`
- Autocomplete support
    - bash-completion, etc.
