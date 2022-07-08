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

    p possible: possible, fixed: fixed, known: known

    result = []
    @v = 10
    find_words2(result, [], Dictionary.instance, possible, fixed, known)
    result
  end

  def find_words2(result, cur, dict, possible, fixed, known)
    @v -= 1
    if fixed.empty?
      p is_it_a_word: cur, dict: dict if @v > 0 || cur[0] == 'v'
      if dict.terminal?
        result << cur.join
      end
      return
    end

    p cur: cur, fixed: fixed, known: known, dict: dict if @v > 0 || cur[0] == 'v'
    l, *fixed = fixed
    if l
      p i_know_this_one: l if @v > 0 || cur[0] == 'v'
      next_dict = dict.lookup(l)
      return unless next_dict
      find_words2(result, cur + [l], next_dict, possible, fixed, known)
    elsif known.size > fixed.select { |x| x.nil? }.size
      p gotta_be_one_of_these: known if @v > 0 || cur[0] == 'v'
      known.each do |l|
        next_dict = dict.lookup(l)
        next unless next_dict
        find_words2(result, cur + [l], next_dict, possible, fixed, known - [l])
      end
    else
      p could_be_any_of_these: possible if @v > 0 || cur[0] == 'v'
      possible.each do |l|
        next_dict = dict.lookup(l)
        next unless next_dict
        find_words2(result, cur + [l], next_dict, possible, fixed, known - [l])
      end
    end
  end
end
