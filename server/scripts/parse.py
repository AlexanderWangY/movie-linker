import csv

with open('moviedata.csv', 'r', encoding='utf-8', errors='replace') as file:
    reader = csv.DictReader(file)
    for row in reader:
        movie = row['Index']

        print(movie)
