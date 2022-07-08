class Guess
  NOT_IN_PUZZLE = "not-in-puzzle"
  CORRECT_POSITION = "correct-position"
  INCORRECT_POSITION = "incorrect-position"

  def initialize(word)
    @letters = []
    next_char_type = NOT_IN_PUZZLE
    word.each_char do |c|
      case c
      when "["
        next_char_type = CORRECT_POSITION 
      when "("
        next_char_type = INCORRECT_POSITION 
      when "]", ")"
        next_char_type = NOT_IN_PUZZLE 
      else
        @letters << [ c.upcase, next_char_type ]
      end
    end
  end

  attr_reader :letters

  def to_param
    @letters.map { |l, r| wrap(l, r) }.join
  end

  def to_s
    to_param
  end

  def with(i, char_type)
    new_guess = self.class.new(to_param)
    new_guess.letters[i][1] = char_type
    new_guess
  end

  private

  def wrap(letter, char_type)
    case char_type
    when CORRECT_POSITION
      "[#{letter}]"
    when INCORRECT_POSITION
      "(#{letter})"
    else
      letter
    end
  end
end
