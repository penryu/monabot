(fn f [line]
  (if (string.find line "attack")
    (case (math.random 20)
          20 "A natural 20!"
          1 "A natural 1..."
          d20 (.. "Attack roll is..." d20))))

{: f}
