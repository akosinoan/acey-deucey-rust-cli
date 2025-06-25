This Program was created while learning Rust programming language.
Acey-Duecey card game with game play implemented on how we used to play it.

Game play:

-ante (if pot money is 0)
-draw 2 cards for each player.
-current player decide if he want to play their card.
  -bet money ( bet <= pot money && bet > 0 )
  -if player cards is a pair
    -player choose higher or lower
  
  -draw card for dealer
    -if player cards is a pair
      -player gueesed correct player win
    -if dealer card in between player card. player wins

  -if player win subtract bet from pot and add to player money
  -if player lose subtract player money and add to pot money
    
  
