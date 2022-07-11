class Rule
  class NotInPuzzle
    def initialize(letter)
      @letter = letter
    end

    def apply(letter, **)
      if letter == @letter
        [false, nil]
      else
        [true, nil]
      end
    end

    def satisfied?(state)
      true
    end
  end

  class Mismatch
    def initialize(letter, position:)
      @letter = letter
      @position = position
    end

    def apply(letter, position:, **)
      if @position == position && @letter == letter
        [false, nil]
      else
        [true, nil]
      end
    end

    def satisfied?(state)
      true
    end
  end

  class Count
    def initialize(letter, count:)
      @letter = letter
      @count = count
    end

    def apply(letter, state:, **)
      if @letter == letter
        [true, (state || 0) + 1]
      else
        [true, state]
      end
    end

    def satisfied?(state)
      !state.nil? && state >= @count
    end
  end

  class Match
    def initialize(letter, position:, **)
      @letter = letter
      @position = position
    end

    def apply(letter, position:, **)
      if position == @position && letter != @letter
        [false, nil]
      else
        [true, nil]
      end
    end

    def satisfied?(state)
      true
    end
  end
end
