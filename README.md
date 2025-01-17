
# newsterm

I don't like modern news apps/websites yet I still like to stay up to date. All I want is a simple TUI that can show me a reverse chronological feed of the latest events from a few different RSS feeds. I also wanted to be able to open the article without having to go to the browser. 

<img width="906" alt="Screenshot 2025-01-17 at 9 08 29 pm" src="https://github.com/user-attachments/assets/a32e666b-9673-4d79-accf-99e8c84330cb" />

This is a simple view of the app. To run it, you need to first create a config with your feeds: 

```
mkdir ~/.config/newsterm
nvim ~/.config/newsterm/feeds
```

You can then add the urls to your RSS feeds like this:

```
http://feeds.bbci.co.uk/news/world/rss.xml
https://www.theverge.com/rss/index.xml
https://www.wired.com/feed/rss
https://www.quantamagazine.org/feed/
https://news.ycombinator.com/rss
https://www.404media.co/rss
https://blog.google/technology/research/rss/
https://www.microsoft.com/en-us/research/feed/
```

You can then `cargo install --path .` in the repo. After that, to run the app, you can type `newsterm` into your terminal. Notes
- I haven't added further configuration except for choosing your feeds
- It's a bit buggy and not feature rich
- There is definitley room for improvement, but I really just wanted a quick thing that could do the job

Key bindings:
- j go down feed/scroll down in article
- k go up feed/scroll up in article
- o open article in web browser
- <Enter> open article in terminal (doesn't always work well)
- <C-u> scroll up a page in article
- <C-d> scroll dwon a page in article
- {1,2,3,4,5,6,7,8,9,0} jump to article (will prompt for full number)
- r reload feed

