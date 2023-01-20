For this assignment, you are going to write a text-based version of
the game "battleship".
If you are not familiar with the game, you can read about it here:

 https://en.wikipedia.org/wiki/Battleship_(game)

before you get started.  Of course, no matter how familiar you are with
the game, please read our description, as we are going to make it slightly
different from normal.


Before we dive into the battleship game you need to write, let us take a
moment to discuss the structure and goals of this assignment:
  (1) To help you think about task breakdown, scheduling,  and planning.
  (2) To bring you up to speed on Java
  (3) To help you see incremental development and testing in action
  (4) To let you see design ideas in practice
  (5) To see how these design ideas work with respect to changing
      requirements.

To acheive these goals, we have set this assignment up with three parts:

Version 1: done as a walkthrough.
     Note that extra credit is available on Version 1 for completing
     intermediate deliverables by specific dates.
Version 2: done on your own
and
Extra Credit: optional, and due later in the semester

We layout the requirements for all of these parts in this document.
We encourage you to read this entire document FIRST.  Note that in the
real world, when requirements are going to change, you do not get to
see "version 2" before you finish version 1.  However, you do here for
the way the learning is structured.

We are going to then walk you through Version 1.  At the start, we are
going to be very prescriptive: telling you exactly what to do and how
to do it.  As we go through Version 1, we will provide less and less
detail with each step.  When you finish Version 1, you will be ready
to start on Version 2, which you will do completely on your own.

Here is what you should do:
 (1) Read this README completely
 (2) Spend 20 minutes thinking about what classes you would make and
     how well they meet the design principles we have discussed.
     This activity is described in more detail at the end of this document
     ("Next Steps" section).
 (3) Read and do the walkthrough.  Note that this walkthrough will take a
     while as it comprises over half of the work in this homework
     assignment. We expect this step to be spread out over a mater of weeks.
 (4) Git push your completed version 1 (after testing etc)
 (5) Do version 2
 (6) Submit version 2 by git push


=================
|   Version 1   |
=================

We are going to play battleship on a text interface with a 20 high by
10 wide grid (though see "extra credit" at the end for discussion of a
graphical interface). Accordingly, we will have coordinates A0 through
T9, like this:
---------------------------------------------------------------------------
  0|1|2|3|4|5|6|7|8|9
A  | | | | | | | | |  A
B  | | | | | | | | |  B
C  | | | | | | | | |  C
D  | | | | | | | | |  D
E  | | | | | | | | |  E
F  | | | | | | | | |  F
G  | | | | | | | | |  G
H  | | | | | | | | |  H
I  | | | | | | | | |  I
J  | | | | | | | | |  J
K  | | | | | | | | |  K
L  | | | | | | | | |  L
M  | | | | | | | | |  M
N  | | | | | | | | |  N
O  | | | | | | | | |  O
P  | | | | | | | | |  P
Q  | | | | | | | | |  Q
R  | | | | | | | | |  R
S  | | | | | | | | |  S
T  | | | | | | | | |  T
  0|1|2|3|4|5|6|7|8|9
---------------------------------------------------------------------------

The game has two players, player A and player B.  While it would be
much nicer to have a networked game where player A and B are at two
different computers, we are not going to make you do that (though see
"extra credit" at the end).  Each player has 10 ships as follows:

2 "Submarines" ships that are 1x2 
3 "Destroyers" that are 1x3
3 "Battleships" that are 1x4
2 "Carriers" that are 1x6

The first phase of the game asks the players to place their
ships. Player A places his ships first, while player B closes her
eyes.  The game should display a blank board (as above), and explain
ship placement as follows:

---------------------------------------------------------------------------
Player A: you are going to place the following ships (which are all
rectangular). For each ship, type the coordinate of the upper left
side of the ship, followed by either H (for horizontal) or V (for
vertical).  For example M4H would place a ship horizontally starting
at M4 and going to the right.  You have

2 "Submarines" ships that are 1x2 
3 "Destroyers" that are 1x3
3 "Battleships" that are 1x4
2 "Carriers" that are 1x6
---------------------------------------------------------------------------

The game should then prompt player A for each ship, e.g.,

---------------------------------------------------------------------------
Player A where do you want to place a Submarine?
---------------------------------------------------------------------------

If the location is invalid (collides with another ship, results in a
ship going off the grid, etc), you should explain the problem to
player, and ask them to place again.  Note that you should ignore case
(so M4H m4h m4H M4h are all the same).  If placement is successful,
you should display the board with the ship on it.  Show an s d b or c
for the ship type respectively.  For example, after a submarine at
M4H, another submarine at A0V and a destroyer at B2V, you would
display:

---------------------------------------------------------------------------
Current ocean:
  0|1|2|3|4|5|6|7|8|9
A s| | | | | | | | |  A
B s| |d| | | | | | |  B
C  | |d| | | | | | |  C
D  | |d| | | | | | |  D
E  | | | | | | | | |  E
F  | | | | | | | | |  F
G  | | | | | | | | |  G
H  | | | | | | | | |  H
I  | | | | | | | | |  I
J  | | | | | | | | |  J
K  | | | | | | | | |  K
L  | | | | | | | | |  L
M  | | | |s|s| | | |  M
N  | | | | | | | | |  N
O  | | | | | | | | |  O
P  | | | | | | | | |  P
Q  | | | | | | | | |  Q
R  | | | | | | | | |  R
S  | | | | | | | | |  S
T  | | | | | | | | |  T
  0|1|2|3|4|5|6|7|8|9
---------------------------------------------------------------------------

After displaying the board, prompt for the next ship.  Repeat this
until all ships are placed.  You should display the board one more
time after the last ship is placed (so with 10 ships,
you will display the board 11 times if there are no errors in
placement).


After doing this for player A, repeat the process for player B (player
A should close his eyes while player B places her ships).  Note that
player A's ships and player B's ships are different "oceans" so player
B will start with a blank board, and can place her ships anywhere
(e.g., even though Player A has a submarine at A0, player B can place a ship
at A0 also, since they are "different" A0s).

Next play begins.   Turns alternate between each player, with player A
going first.  First, display the board to player A.   You will display
two things: player A's ships and what hits/misses player A has already
made.  In this display, you will use the same layout/symbols as above,
but with * to indicate a hit portion of your own ship, and X to
indicate a miss on an enemy ship.  For example:

---------------------------------------------------------------------------
Player A's turn:
     Your ocean                           Player B's ocean
  0|1|2|3|4|5|6|7|8|9                    0|1|2|3|4|5|6|7|8|9   
A s| | | | | | | |c|  A                A  | | | | | | | | |  A
B s| |d| | | | | |c|  B                B  | | | | | | | | |  B
C  | |*| | | | | |c|  C                C  | |X| | | | | | |  C
D  | |d| | | | | |c|  D                D  |X|d|d| | | | | |  D
E  | | | | | | | |c|  E                E  | |X| | | | | | |  E
F  | |d| | | | | |c|  F                F  | | | | | | | | |  F
G  | |d| | | |b| | |  G                G  | | | | | | | | |  G
H  | |d| | | |b| | |  H                H  | | | | | | | | |  H
I  | | | | | |b| | |  I                I  | | | | | | | | |  I
J  | | | | | |b| | |  J                J  | | |X| | | | | |  J
K c|c|c|c|c|c| | | |  K                K  | | | | | | | | |  K
L  | | | | | | | | |  L                L  | | | |X| | | | |  L
M  | | | |s|s| | | |  M                M  | | | | | | | | |  M
N  | | | | | | | | |  N                N  | | | | | | | | |  N
O  | | | | | |b| | |  O                O  | | | | |s|s| | |  O
P  | | | | | |b| | |  P                P  | | | | | | | | |  P
Q  | | | | | |b| | |  Q                Q  | | | | | | | | |  Q
R  | | | | | |b| | |  R                R  | | | | | | | | |  R
S  | | | | | | | | |  S                S  | | | | | | | | |  S
T d|d|d| | | | | | |  T                T  | | | | | | | | |  T
  0|1|2|3|4|5|6|7|8|9                    0|1|2|3|4|5|6|7|8|9
---------------------------------------------------------------------------

In the above example, player A has successfully hit player B's ships at
four locations:

O5: a submarine
O6: a submarine
D2: a destroyer
D3: a destroyer

and missed player B's ships at D1, C2, E2, J3, and L4 (meaning no
ships are located there)

Player A's own ships have been hit once, at C2.

Player A is then prompted for the coordinate to fire at.   If the
coordinates are invalid, the game should prompt player A to enter
a valid choice. The game will then report the result, such as

---------------------------------------------------------------------------
You hit a carrier!
---------------------------------------------------------------------------
or
---------------------------------------------------------------------------
You missed!
---------------------------------------------------------------------------

Play then proceeds with Player B's turn.   The boards are displayed
for Player B (so Player B's state is shown on the left with all of B's
ships known, and Player A's ocean is shown on the right with only
hits/misses known).

Play continues to alternate until one player has no more ships.  When
one player has no more ships, the other player wins.  Your game should
print a message stating who won, and then exit.

Before proceeding to Version 2, please check that...
   o You have met all requirements for Version 1
   o You have rigorously and thoroughly tested your code
   o You have commented your code well
   o You have made a git commit which contains your version 1 code
     (i.e. "delivered" version 1 to your client)

=================
|   Version 2   |
=================


After completing version 1 of battleship, your customer changes the
requirements slightly, and you now need to make version 2.   There are
three changes to the requirements:


 (1) Changes to ship design.
     To spice the game up a bit, the customer decided that there
     should be some different ship shapes that are not just
     rectangular.


     In particular, each player now has the following ship types:

        - 2 "Submarines" that are 1x2 rectangles (represented by "s")
        - 3 "Destroyers" that are 1x3 rectangles (represented by "d")
        - 3 "Battleships" that are now shaped as shown below

               b      OR    b         bbb         b
              bbb           bb   OR    b     OR  bb
                            b                     b

               Up          Right      Down      Left
        - 2 "Carriers" that are now shaped as shown below
        
                  c                       c             
                  c           cccc        cc         ccc
                  cc   OR    ccc      OR  cc   OR  cccc     
                  cc                       c         
                   c                       c
                   
                 Up           Right     Down          Left
      Note that the submarines and Destroyers are unchanged, and still
      have horizontal (H) and vertical (V) orientations.
      
      Battleships and Carriers now have new shapes, and 4 orientations
      (as pictured above).  These four orientations are now up (U),
      right (R), down (D), and left (L), with each diagram above
      having its orientation written below it.

      Accordingly, placing a Battleship or a Carrier requires
      coordinates that end with U,L,R, or D instead of H or V.
      Note that the coordinates still name the top left of
      the ship.  With ships that are not rectangular, we define
      this top left corner to be the corner of the smallest rectangle
      which fully encloses the ship.  To clarify this last point,
      we re-draw each ship with the top-left corner indicated by
        - A capitalized letter when it is part of the ship
        - A star (*) when it is not part of the ship.

         Battleships:
              *b      OR    B         Bbb        *b
              bbb           bb   OR    b     OR  bb
                            b                     b

               Up          Right      Down      Left
               
         Carriers:
                  C                       C             
                  c          *cccc        cc       * ccc
                  cc   OR    ccc      OR  cc   OR  cccc     
                  cc                       c         
                   c                       c
                   
                 Up           Right     Down          Left

   (2) New moves.
       A player now has 2 new actions, each of which is limited to
       3 uses per game.  At the start of player's turn, the game
       should display the board, prompt for which action type the
       player wants to use, and then prompt for any information
       needed for that action type.  For example:
---------------------------------------------------------------------------
Possible actions for Player A:

 F Fire at a square
 M Move a ship to another square (2 remaining)
 S Sonar scan (1 remaining)

Player A, what would you like to do?
---------------------------------------------------------------------------


       Note:  once a player is out of their 3 uses of a the new
       moves you MAY (but do not have to) omit that item from the menu
       entirely.  You MAY also, if all 3 uses of both new actions
       are used up, go directly to the "fire" action as in version 1.
       
            (2a)  Move ship.
                  When using this move type, a player selects one of
                  his/her own ships (the game should prompt for which
                  ship, and the player should be able to type any
                  coordinate which is a part of the ship they want).
                  The player then is prompted for a new placement
                  (location + orientation, as in initial placement).
                  The ship is then moved to that position.  Any
                  existing damage to the ship remains in the
                  same relative position(s) of the ship.

                  If the player selects an invalid location,
                  the player is re-prompted for their actions (and
                  might select move again, or might select a different
                  action).
 
                  Note that the other player's display does NOT gain
                  any information that the ship moved.  If they
                  previously hit that ship, the hit markers remain.
                  If they previously missed the new location, those
                  markers also remain.

           (2b)   Sonar scan.
                  When the player selects this move type, the game
                  prompts for the center coordinates of a sonar scan.
                  Any coordinate on the game board is valid, even
                  if part of the scan will go off the edges of the
                  board.

                  The game then considers the following pattern
                  around (and including) the center (C)

                                    *
                                   ***
                                  *****
                                 ***C***
                                  *****
                                   ***
                                    *

                 and reports on the number of squares occupied by each
                 type of ship in that region.   For example:


---------------------------------------------------------------------------
Submarines occupy 2 squares
Destroyers occupy 0 squares
Battleships occupy 5 squares
Carriers occupy 1 square
---------------------------------------------------------------------------

                 Note that no information about the exact position of
                 any ship in that region is report.  There is also no
                 information about how many of each ship---only how
                 many squares are occupied.  We do not know if 2
                 squares occupied by submarines is one submarine
                 entirely inside the region, or 2 submarines with
                 one square in the region and one square outside of
                 the region.

    (3) Play against computer.
        When the game starts, it should prompt the user for whether
        each of Player A and/or Player B is a human player or to be
        played by the computer (Note that any combination is valid:
        human vs human, human vs computer, computer vs human, or
        computer vs computer).

        Your computer play does NOT need to be very smart---you are not
        expected to develop any sophisticated AI.   Your computer
        player can just place all of its ships in pre-specified or
        random locations (as long as they are on the board) and
        can fire in a simple pattern (iterate through all squares in order).
        Note that we would be ok with random behavior too, but students
        sometimes find testing random behavior difficult.
        However, if you want to make a fancier computer
        player see "extra credit" at the end of this document.

        When the computer plays its turn, no board state should
        be displayed and no prompts should be printed.   The game
        should only print the outcome of the action (e.g.,
        Player B hit your submarine at D4!).

        If the computer plays a special action, the game should state
        "Player B used a special action" but not state which one nor
        any details of where or its outcome.  Your computer player
        is not required to use special actions, but may do so if it wants.


                 
=================
| Extra Credit  |
=================
  We note there are two distinct (and very different) extra credit
  opportunities on this homework. One is to do phase 1's goal on a
  schedule specified in that overview.   The other (described here) is
  to add extra features, and is due much later in the semester.
                 
  The above "game" is a nice activity in exploring changes to software
  requirements/design, but a pretty terrible game (trying to play
  battleship with two people using one computer, and trying not to
  accidentally see something you should not does not really work well).

  ******************************************************************
  *** Note that the extra credit is NOT due with the assignment, ***
  *** but rather may be done after the deadline.                 ***
  *** You may turn in any extra credit by April 21st             ***
  *** Please notify your project TA if/when you submit this      ***
  *** extra credit                                               ***
  ******************************************************************



  We provide you with 3 opportunities for extra credit.  You may do
  any combination of these (one, two, or three of them).  These are
  rather open-ended, so the number of points gained will be decided
  based on the quality of the work.



  (1) Graphical User Interface.
      Replace the text-based interface with something graphical.
      A more polished, more intuitive UI will gain more points here.
      Note that this may be any kind of UI: Desktop, Web, or Mobile.
      
  (2) Networked game.
      Make the game so that the two players can be on different
      computers.
      You can explore a variety of features in this area from
      simply using game play, but across a network, to adding
      any number of features: user accounts/authentication,
      match making, etc.  


  (3) Intelligent computer play.
      Instead of just having the computer fire in a fixed pattern, write an
      intelligent AI.  This could be small/simple improvements
      (making use of special moves, using information from the
      outcome of each shot to inform your next decisions) for
      a few points, or it could be something very clever and
      sophisticated for many.


For any of these, what you add, how you add it, and anything else you
want to improve about the game is up to you!

================
| Next Steps   |
================

Before proceeding to the walkthrough for version 1, we want you to take
about 20 minutes to think about the design of Battleship Version 1.

  1. Start by thinking about what classes and interfaces you might
     need (you might want to skim back through this README as you do so).
     Write down the classes you come up with.  Remember that classes
     should be nouns. Now, think about what your classes have
     in terms of fields and methods, as well as their relationships
     to each other:
        a. Write down the major methods of each class.  You shouldn't
           try to write out the full method signature, but just
           a few-words description of what it does (e.g,
           "check for winning/losing").
        b. Write down the fields of the class.  Think about
           whether these fields are primitives, or other classes
           (has-A relationships)
        c. Note down any is-A (inheritance) relationships.

  2. Next, look at your list of classes and think about the design
     principles we
     discussed in class.
        a. For each class, does it obey Single Responsibility Principle?
        b. How tightly coupled are your classes to each other?
        c. Do your classes follow Dependency Inversion?
           If not, what interface might be appropriate to introduce?
        d. Do your interfaces obey the Interface Segregation Principle?
           If not, how should they be divided?
        e. Do you expect to have any code duplication (violating DRY)?
        f. Do your classes obey the Open/Closed Principle?
           In particular, how hard would it be to change your code
           for Version 2?
           
At this point, we recommend stepping away from this assignment
for a while (your brain is surely worn out on battleship at the moment),
and coming back to it later.  When you come back, read over the
design you came up with to bring it back into your brain (have your
thoughts on it evolved while you were away?  if so, that is ok).

After you have refreshed your memory on what you came up with,
open v1Overview.txt and read it. We'll walk you through Version 1,
starting with our "think aloud" on this high-level design process.



 


           
                  
                  
                  