Meetup Find Events RSS
======================

Generates an RSS feed of upcoming events on [Meetup][1]. This makes it easier to
be informed of meetups even in groups you don’t participate in.

The program is designed to run locally at a recurring interval (via `cron` or
`launchd` for example), generating an XML feed file.


## Usage
The following shell script generates a new feed and writes it to the
`meetup-upcoming.xml` file:

	#!/bin/sh

	meetup-find-events-rss \
		--meetup-api-token YOUR_TOKEN \
		--latitude 48.8692 \
		--longitude 2.34777 \
		--end-date 2018-04-15 \
		> ~/meetup-upcoming.xml

A Meetup API token can be obtained from
<https://secure.meetup.com/meetup_api/key/>.


## Example
Here's a demonstration of the result of the above script in Newsboat.

	echo "file://$HOME/meetup-upcoming.xml" >> ~/.newsboat/urls

[TODO demo gif]


## Install
A binary built for Mac OS X is available on the [releases][2] page. Download the
binary and put it in your `PATH`.

To compile from source:

	$ cargo install --git https://github.com/teddywing/meetup-find-events-rss.git --root /usr/local


## Uninstall

	$ cargo uninstall --root /usr/local meetup-find-events-rss


## License
Copyright © 2018 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).


[1]: https://www.meetup.com/
[2]: https://github.com/teddywing/meetup-find-events-rss/releases
