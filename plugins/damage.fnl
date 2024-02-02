(fn roll [a x]
  (case [(tonumber a) (tonumber x)]
        [a x] (fcollect [r 1 a] (math.random x))
        [nil x] (roll 1 x)
        default "other"))

(fn f [line]
  (case (string.match line "damage (%d*)d(%d+)")
        (a x) (let [rolls (roll a x)
                    total (accumulate [sum 0 i n (ipairs rolls)]
                            (+ sum n))]
                (.. a :d x " => " (table.concat rolls " ") " => " total))))

{: f}
