# Overview

TODO

# Input

The input shall be a TOML file, as they are easy to read and write.

```toml
[blockprofile.userA]
macs = [ "aa:bb:cc:dd:ee:ff", "12:34:54:67:89:ab" ]
always_block = ["facebook.com", "yahoo.com" ]
intermittent_block = [ 
    { host = "reddit.com", length = "15min" }
      host = "youtube.com", length = "45min", day_total = "2h" }
    ]
block_times = {
    name = "Go to sleep",
    start = "11:00 pm",
    end = "6:30 am",
    exceptions = [ "websites.wikipedia", "audible.com" ]
}


[websites.facebook.com]
alias_domains = [ "content.facebook.com", "www.facebook.com", "facebook.com" ]

[websites.wikipedia]
alias_domains = [ "es.wikipedia.org", "la.wikipedia.org", "en.wikipedia.org" ]
```

# API
There shall be an API call that can be made from my phone to enable a block profile
for a given amount of time. This, I think, comes first. The intermittent blocking
is a bit of a more advanced feature that requires querying data dynamically.

----
As you can see, I want a way to block certain websites, but not necessarily all
websites A) during specific times, and B) when I'm feeling like I'm throwing away
a lot of time, or C) when I'm using specific websites too much.

