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

    result = []
    find_words2(result, [], Dictionary.instance, possible, fixed, known)
    result
  end

  def find_words2(result, cur, dict, possible, fixed, known)
    if fixed.empty?
      if dict.terminal?
        result << cur.join
      end
      return
    end

    l, *fixed = fixed
    if l
      next_dict = dict.lookup(l)
      return unless next_dict
      find_words2(result, cur + [l], next_dict, possible, fixed, known)
    elsif known.size > fixed.select { |x| x.nil? }.size
      known.each do |l|
        next_dict = dict.lookup(l)
        next unless next_dict
        find_words2(result, cur + [l], next_dict, possible, fixed, known - [l])
      end
    else
      possible.each do |l|
        next_dict = dict.lookup(l)
        next unless next_dict
        find_words2(result, cur + [l], next_dict, possible, fixed, known - [l])
      end
    end
  end
end
