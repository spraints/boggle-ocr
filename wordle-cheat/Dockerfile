FROM ruby:3.0.1

WORKDIR /app
COPY Gemfile /app
COPY Gemfile.lock /app
RUN bundle config set --local without 'development test' && bundle install
COPY . /app
RUN bin/rails assets:precompile
ENV RAILS_ENV=production
ENV RAILS_LOG_TO_STDOUT=true
ENV RAILS_SERVE_STATIC_FILES=true
CMD ["bin/rails", "server"]
