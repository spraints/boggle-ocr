class TheCheatController < ApplicationController
  def index
    @guesses = []
    if prev = params[:previous_guess]
      prev.each do |word|
        @guesses << Guess.new(word.to_s)
      end
    end
    if guess = params[:guess]
      @guesses << Guess.new(guess)
    end
    @remaining_words = find_words(@guesses)
  end

  private

  def find_words(guesses)
    return nil if guesses.empty?

    possible = ('a'..'z').to_a
    fixed = [nil] * 5
    known = []
    guesses.each do |guess|
      guess.letters.each_with_index do |(letter, char_type), i|
        letter = letter.downcase
        case char_type
        when Guess::NOT_IN_PUZZLE
          possible.delete(letter)
        when Guess::CORRECT_POSITION
          fixed[i] = letter
          known.delete(letter)
        when Guess::INCORRECT_POSITION
          known << letter
        end
      end
    end

    find_words2 \
      possible: possible,
      fixed: fixed,
      known: known
  end

  # Finds words given constraints.
  #
  # possible - Array of all allowed letters.
  # known - Array of all letters that must be in the puzzle.
  # fixed - Array, same length as 'work'. Any letters that are filled in are copied into the same place in 'work'.
  #
  # found - Array, will have any new words added to it. Will be returned.
  # work - Array, will be modified. Using this saves us from needing to allocate.
  # len - Number of letters already placed into 'work'.
  # dict - Dictionary or Node that can be used to find the next letter.
  def find_words2(possible:, known:, fixed:, found: [], work: ['a']*5, len: 0, dict: Dictionary.instance)
    if len == 5
      if dict.terminal?
        found << work.join
      end
      return found
    end

    i = len
    if l = fixed[i]
      if next_dict = dict.lookup(l)
        work[i] = l
        find_words2 possible: possible, known: known, fixed: fixed,
          found: found, work: work, len: len + 1, dict: next_dict
      end
    elsif choices = must_be_a_known_value(known: known, fixed: fixed, work: work, len: len)
      choices.each do |l|
        if next_dict = dict.lookup(l)
          work[i] = l
          find_words2 possible: possible, known: known, fixed: fixed,
            found: found, work: work, len: len + 1, dict: next_dict
        end
      end
    else
      possible.each do |l|
        if next_dict = dict.lookup(l)
          work[i] = l
          find_words2 possible: possible, known: known, fixed: fixed,
            found: found, work: work, len: len + 1, dict: next_dict
        end
      end
    end

    return found
  end

  def must_be_a_known_value(known:, fixed:, work:, len:)
    remaining = work.size - len
    fixed.each_with_index do |l, i|
      remaining -= 1 unless i < len || l.nil?
    end
    unused_known = known - work[0, len]
    if unused_known.size < remaining
      return nil
    end
    return unused_known
  end
end
