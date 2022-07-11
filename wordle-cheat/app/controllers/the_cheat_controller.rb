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
    @remaining_words = Guesstimator.guess(@guesses)
  end
end
