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
    unless @guesses.empty?
      @remaining_words = Guesstimator.guess(@guesses)
      @letter_counts = count_letters(@remaining_words)
    end
  end

  private

  def count_letters(words)
    counts = Hash.new(0)
    words.each do |w|
      w.chars.uniq.each do |c|
        counts[c] += 1
      end
    end
    counts
  end
end
