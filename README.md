# find-me-a-xbox
A program that checks xbox availability every 15s, adds to cart when available, then texts your phone.

This is a little project I decided to make for fun. I doubt it will actually work because of how scarce the product is and how volatile web scrapers are without constant maintenance... but as a starting point I have a program that checks Best Buy as a logged in user. I decided to add the log in step as the initial step so that when the Xbox is added to your cart it and you are texted, it should show up on your phone when you navigate there from the link. Anyway, here's to hoping and having fun with an annoying situation 🥂

## Getting Started
Assuming you have rust installed, all you'd need to do is:

- Create a Twilio trial account (or paid one but they give you $15 for free)
- Create a `.env` file with all the proper variables
- `cargo run`
