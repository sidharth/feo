---
slug: what-i-learned-at-the-inclusive-stem-hackathon-iiit-b
title: What I learned at the Inclusive STEM hackathon @ IIIT-B
published: true
publish_date: 2018-01-29
---

Last weekend, I had the opportunity to participate in a different kind of hackathon. What made this one different? This one was specifically geared towards the inclusion of visually challenged people in STEM fields. So a fundamental premise for this effort was that each team would be composed of ~2 visually impaired individuals, ~2 industry professionals and a student from IIIT-Bangalore.

*It was an incredible experience*. For the first time in my life, I saw screen readers being used as a tool, not simply as a novelty. Tools like JAWS/NVDA on laptops, and VoiceOver/Talkback on mobile are indispensible.

The problem we chose to work on

After a detailed discussion, we chose to focus on cash money handling and identified two major problem points faced by blind individuals:-

- The first is that it is difficult for blind people to identify the denomination of Indian currency note by touch - more so apparently for the new note designs post demonetization[1].
- In addition to this, another related problem area was to keep track of the total money and distribution of various denominations in their wallet.

The solution

To solve this, we built a currency identification app - CurrenSee; and we gave it the following features:-

- It should be able to identify the different Indian currency notes using only the smartphone camera. 
- It should perform the analysis on the image-stream directly without the use of an explicit “capture image” button.
- Wallet maintenance - after detecting image, give an option to add/deduct the amount from wallet
- Offline accessibility - no internet connection should be required
- Should gel well with Talkback

We experimented with Microsoft’s and Salesforce’s platforms for training the neural networks online for this classification task. For even a relatively small dataset of ~200 labelled images per class that we had created, the performance of the resultant model was decent on empirical testing[2].

Android Accessibility learnings:-

These are essentially all things that Google already advocates as best practices for accessibility, but here it goes:-

- The easiest change you can make to your Android app to improve accessibility would be to use the contentDescription attribute of Android Views to make the Talkback text more descriptive. For us, even this simple step made the app seem to flow more naturally.
- Effectively triggered accessibility event readings describing the system state (such as the wallet balance in our case) can be quite useful.
- Another easy accessibility win is to minimize the number of elements in the screen as much as possible. Talkback users generally go through the items on the screen by swiping left on the screen (to go to previous item), or swiping right (to go to the next one). Having a larger number of views would mean having to swipe through an inordinately large number of views. (If that isn’t possible, then one should create natural groupings of units of information using the focusable=true XML property. This would ensure that the grouped content is read together.)

Other observations and thoughts:-

- Even in larger cities, many doctors aren't aware of the existing state of the art in assistive technology that was made possible in the last few years.
- A significant percentage of our higher educational institutes are not equipped to cater to visually impaired students. Impairment ≠ Disability. How can we further leverage technology to improve how education is imparted?

![image](./assets/hackathon-iiitb-gang.jpeg)

^Our awesome team in action:- (Left to Right) Abhishek, me, Tarun, Kartik, Shashi

[WIP] Github :- https://github.com/sidharth/currensee

[1] - I initially thought the new notes would be easier to identify due to the marked lines on the sides of the note, but apparently those embossings get worn out and flattened very quickly rendering them effectively useless.

[2] - Caveat - At one point we realized that it was classifying anything with a white background as a Rs 2000 note because the training for that note was mostly done on a white background. When we made the training set more diverse, this issue was resolved.