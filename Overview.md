# Dreams: An Overview
By: ME! a solo dev

### 1. Core Identity & Gameplay Loop

##### Genre: 2D Singleplayer Top-Down Procedural Survival RPG 

##### Core Gameplay:
This is a 2D Singleplayer Top-Down Procedural Survival RPG, where the player explores an open, procedurally generated world, engaging in real-time combat, gathering resources, completing quests, and interacting with NPCs, and uncover the secrets of this world. Each in-game day lasts approximately 30 minutes and ends at Midnight, where the entire world resets with a new procedural seed.

##### The Midnight Reset:
The world is a fractured mind attempting to reconstruct the reality it knows. Each reset is the brain trying to “solve” itself. The player is unknowingly disrupting this and causing this process to fail every cycle.

Player progression persists across loops (gear, stats, items), while the world, NPC placements, and terrain regenerate. Key structures and story elements are guaranteed to reappear in different forms each cycle.

##### Short-Term Player Motivation:
Each cycle encourages exploration, resource gathering, quest completion, and character strengthening under the belief that the player is progressing toward saving the world.

---

### 2. Systems & Mechanics

#### Progression (The “Virus” System):
The player collects weapons, food, and survival tools, alongside unique “memory fragments” tied to story progression. While these function like traditional RPG upgrades, they represent pieces of a traumatic memory.

As the player becomes stronger:

* The world becomes more unstable and distorted
* NPC dialogue becomes less coherent or more fragmented
* Environments may generate in harsher or more broken forms
* Systems subtly deteriorates, reflecting growing strength of the trauma

This creates a hidden cost to progression: the stronger you are, the more you beak the world. I have yet to solved the fact that this is against how you need to be stronger to survive and progress. Possibly making it so something feels wrong but the player needs the power.

#### Emotion System:

A central Emotion (e.g., Anger, Sadness, etc.) influences each world reset. It is basically just how the Dreamer/mind feel.

It affects:

* World generation (layout tone, density, hostility or how dangerous the would would be)
* NPC behavior and dialogue tone
* Enemy aggression and environmental conditions

Players can influence the Emotion through actions and interactions, indirectly shaping the next cycle’s world state.


#### NPC Interaction System:

NPCs are fragmented memories or immune responses within the mind.

* Interaction is based on free-form typed input
* The system parses keywords and emotional tone
* NPCs generate responses dynamically using keywords and contextual phrase pools

There would be a guide and I am facing a lot of problems, it can be found [here](https://github.com/OwenOwen76/NPC_Dialog_System).

NPC behavior is influenced by the Emotion, among other things.

---

### 3. Narrative Structure

#### The Player:
The player believes they are the hero attempting to fix a broken world.

In reality, they are a memory of a real-life attacker who is the source of the Dreamer’s trauma.

This is progressively shown, such by:

* NPC reactions/dialog
* Helping someone may cause unease or unclear consequences
* Dialogue contains contradictions that are easy to dismiss early on
* The world's response to various activities


#### The Girl:

A sarcastic, mean guide who introduces mechanics and systems. Also the Dreamer's mind.

* Represents the dreamer’s mind/subconsciousness
* Frequently insults or undermines the player

Her role is to:

* Deliver tutorials
* Provide commentary
* Represents the Dreamer's subconsciousness while she is in a coma, (a filter/shield?)


#### The Dreamer

Everything happens in her mind while she is in a coma.

* Faced ______ and lost her family/loved ones, in a coma, dealing with the grief and the traumatic memories
* Physical version of the Girl

---

### 4. The Final Boss

#### The Mirror Boss:

The final boss is a original version of the player character.

* Represents the true, original memory of the attacker
* Uses the player’s accumulated gear and stats
* Scaled to be stronger for balancing purposes, but also to reflecting the full weight of progression

---

### 5. Endings

#### Victory:
The player defeats the original version.

* The traumatic memory become too much
* The world collapses, I have an idea for gameplay reason where there'll be exact 30min countdown before the world is gone, during which there's special enemies/bosses
* The Dreamer dies, this should end with, after the 30min countdown, a cardiac monitor showing the heart stops, either fades out or have something after.
* I think I might change this, it's too black-and-white for this game

#### Sacrifice:
The player allows the original version to erase them.

* The Dreamer accepts the past and moves on
* The world stabilizes
* The Dreamer wakes up from her coma, maybe end with her friend rushing in to hug her. Lost loved ones but not alone?

#### More?
Maybe a few more on NG+ of something like that, on the to-do list.

#### New Game +
Possible option on the horizon. 
* Unlocked after finishing the Victory ending?
* Overall more intense, maybe some extra enemies & bosses
* The Girl now:
  * Remember what you did last time
  * "Back here again? Was destroying the world once not enough for you?" (possible introduction idea)
  * A story boss, fought from time to time based on story events or maybe just jumps you more and more as the story progress.


---

### 6. Technical Overview

* Engine: Bevy (Rust)
* World Generation:

  * Terrain via Perlin Noise:
  
      * Terrain generation breaks down at about 10,000,000 blocks away from spawn, and the entire game breaks at about 100,000,000 blocks away from spawn
      
  * Structures via Wave Function Collapse (WFC)

* TO BE UPDATED
---

### 7. Current Progress

* 2 month into development working on a MVP
* Full single biome (forest) noised, with a player complete and full camera complete
* Combat and collision is being worked on right now
* NPC dialog system has a 1-NPC MVP version that read limited keyword and creates dynamic response, this is very much a work in progress and is much harder than I thought
* There a fully functional pause menu and loading screen because why not
* A web version (very bad, outdated) [here](https://owenowen76.github.io/)
* TO BE UPDATED
