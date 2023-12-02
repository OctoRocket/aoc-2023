R ← "one|two|three|four|five|six|seven|eight|nine"
O ← {"1""2""3""4""5""6""7""8""9"}
Ro ← ⊂:"|\\d"R
Rt ← ⊂⇌R"|\\d"
M ← ≡⊡⊙¤⊡1⍉⊚⍉:⊂.+1⇡9⊞∊⊂O⊐⊜∘≡≠"|".R
C ← /+≡(parse/⊂∵$"_")⍉⊟:∩M∵⇌∩(∵⊐⊢⊐≡⊐regex)¤Rt∵⇌,¤Ro
