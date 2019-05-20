### What is this?

An http server that listens for requests to 'http://127.0.0.1/q?=YourQueryHere' and responds with an opinionated formatted version with html syntax highlighting.

### Example

Take the following as input:

``sELECT users.`UserID`id,now(+6365)
fRoM`users`
JOIN`companies`using(`CompanyID`) WHERE`users`.`Email`='プログ\'ラマーズ>'`companies`.`NetworkID`=x'1541A488C87419F2' and`companies`.`NetworkID`=0x1541A488C87419F2
and`companies`.`CompanyID`in(@@AdminCompanyIDs) and yeet.poop   in('') and`users`.`__Active`<>0.0 and @i := -.232 order by`users`.`__Added`desc limit 1;``

Well that will spit back out something that looks like this

![](https://d159l1kvshziji.cloudfront.net/i/BUI3/M.png)

As you can see, this is how *I* like to format my queries, this certainly doesn't try to format them anything remotely close to how something like MySQL Workbench likes to format them (which is awful IMO).

### Notes

Notice in this doc at the very beginning it says "127.0.0.1" and not "localhost". In my testing, localhost still has to do a look up of some sort and does actually add some measurable time to making requests to this server. Only downside is that if you jump to IPV6, we need to know what to change this to (localhost will work for both IPV6 and IPV4)
