FROM ruby:3.0.1

WORKDIR /app
COPY Gemfile /app
COPY Gemfile.lock /app
RUN bundle config set --local without 'development test' && bundle install
COPY . /app
ENV RAILS_ENV=production
ENV RAILS_LOG_TO_STDOUT=true
RUN bin/rails assets:precompile
CMD ["bin/rails", "server"]
