# girl.technology

This is the source code for the website <https://girl.technology>.

## Goals
* [x] Link directory for personal websites
* [ ] Automated registration
* [ ] Automated web graph creation

### Link Directory
Users of the website will be able to see categories, such as dog, cat, fox,
etc. of different personal websites. For example, the website
<https://wolfgirl.dev> would be listed under <https://wolf.girl.technology>.
The purpose of this is to let users discover other websites or blogs similar to
their interests.

### Automated Registration
Initially, users will have to put their links in the database by contacting the
site owner. However, self-serve mechanisms for adding sites are possible. One
idea is to have users place their configuration in a
`/.well-known/girl.technology` file. Users could request a scan once per
minute to add their site, and an automated scan will happen once per day to
remove any dead sites.

### Automated Web Graph Creation
Going beyond web rings, it would be interesting to see the different
relationships between users' personal sites, for both informative and ranking
purposes if the site gets popular. A web spider would crawl sites for outgoing
links to other sites in the directory for a fully-automated process.
