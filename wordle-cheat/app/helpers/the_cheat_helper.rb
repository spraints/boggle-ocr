module TheCheatHelper
  def updated_guess_url(new_char_type, guess_idx, letter_idx)
    url_for(previous_guess: updated_guesses(new_char_type, guess_idx, letter_idx))
  end

  def updated_guesses(new_char_type, guess_idx, letter_idx)
    new_guesses = @guesses.dup
    new_guesses[guess_idx] = @guesses[guess_idx].with(letter_idx, new_char_type)
    new_guesses
  end
end
