# Rustblock

Rustblock helps you take your life back from your devices. The ultimate goal is
to throttle or impose blocks on time-wasting or other undesirable websites after
a configurable amount of time.

## The Problem.
As far as I am aware, all existing website blocking solutions either focus on
totality or blocking outside of specific times. These are better than nothing,
but they are simply inadequate. I don't want to **never** scroll Reddit. I just
don't want my primitive monkey brain making it hard to do things I *actually*
want to do because it's jonesing for a hit of dopamine. I don't only want to
relegate social media time to between hour X and hour Y, because sometimes there
are legitimate reasons for accessing those sites. I use Reddit frequently in
my research, for instance. And occasionally I'll give in to the primitive monkey
brain and say, "Alright, just 5 minutes and then back to my project."

The effect of this is always the same: I end up disabling the time guards, and
they don't come back on their own.

## The Solution
The solution is not to block things in totality, but to introduce friction, so
monkey brain learns that this is not an always-available way to get a dopamine fix,
but is something that takes effort to use, and has limited duration. You know,
just like everything else in this world before Big Tech hijacked our brains'
reward systems.

Rustblock's ultimate goal is to monitor connections your devices have with
"undesirable" websites and intermittently cut them off. Say you're scrolling
Bluesky for 6 minutes and suddenly your feed stops refreshing. That is the most
effective message I can imagine to send to the monkey brain to get it to release
its stranglehold on our web browsing habits.

I believe this will have a dual effect. First, it makes it easy to interrupt the
doomscrolling I and so many others are prone to. The primitive frustration and
boredom that comes with losing that Internet access has a really great "snap
out of it" effect. Second, the interruption is guaranteed not to be permanent.
It eliminates in some very important ways the scarcity mindset, so instead of
overindulgence feeling necessary, it feels more superfluous.

There is another part to this. Some days your brain just isn't doing so great and
it needs a total detox. Rustblock does that too. Set up as shortcut on your phone
(or with a networked ESP32, what have you) to SSH into your router and enable
a block profile for a day at a time. You can surely go a day without your
favorite website, right? In any case, it will be really good for you.

## The Current State of Affairs
These ideas are, of course, mostly unproven. I have currently only implemented
the last feature, the total "emergency" cutoff, as that was the simplest thing
to implement first. The goal is to expand this project to include the other
features if I get the chance.

# Setup
If someone else has any interest in using this, I'm happy to put more details here.
Drop me a line on [Mastodon](https://fosstodon.org/@steelswords) and I'll gladly
help you out.

But for the time being, it would just be a security risk divulging network details
just to not spend my time coding, so this will remain sparse.

I will say I have this built for DD-WRT running on a more modern Netgear router.
I have a `router` host defined in my `~/.ssh/config` to make scp, and thus `./buildanddeploy.sh`
work nicely. Edit the target in that script and you should be good.

I use [cross](https://github.com/cross-rs/cross) to cross compile this project
for the router. Check out their website for details on setup.


# Help

## SCP Failed 
use `scp -O foo bar baz`. This enables the legacy SCP protocol instead of the SFTP
protocol. Sometimes DD-WRT is just kinda old that way.

# License

This project is licensed under the [Giveback License 0.1](https://github.com/steelswords/Giveback-License).
Any modifications you make must be made publicly available.
